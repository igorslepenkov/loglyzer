use rayon::prelude::*;
mod args;
mod errors;
mod styler;
mod utils;

use crossbeam::{channel::unbounded, sync::WaitGroup};
use std::{
    fs::{self, File, ReadDir},
    io::{prelude::*, stderr, BufReader, Write},
    path::PathBuf,
    sync::Arc,
    thread::scope,
};

pub use errors::AppError;
use jmespath::Expression;
use serde_json::Value;

use crate::{
    args::{Args, ParsedArgs},
    utils::{check_dir_existence, get_file_created_date},
};
use crate::{
    styler::print_prittified_logs,
    utils::get_available_cpus_count,
};

pub use crate::utils::report_error;

type FileProcessingResult = Vec<String>;

pub fn run() -> Result<(), errors::AppError> {
    let args = Args::init().check()?;

    let args = args.parse_args();

    let sort_expr = Arc::new(args.sort_by.clone());

    let mut all_lines = get_log_lines(&args, &args.filter);

    if let Some(ref sort_expr) = *sort_expr {
        all_lines.sort_by(|a, b| {
            let val_a =
                serde_json::from_str::<Value>(a).ok();
            let val_b =
                serde_json::from_str::<Value>(b).ok();

            let key_a = val_a.as_ref().and_then(|v| {
                sort_expr.search(v.clone()).ok()
            });
            let key_b = val_b.as_ref().and_then(|v| {
                sort_expr.search(v.clone()).ok()
            });

            key_a.cmp(&key_b)
        });
    }

    print_prittified_logs(all_lines);

    Ok(())
}

fn process_log_file(
    path: &PathBuf,
    filter: &Arc<Option<Expression>>,
) -> Result<FileProcessingResult, AppError> {
    println!("Processing {}", path.to_string_lossy());

    let file = File::open(&path)?;
    let file_reader: Box<dyn BufRead> =
        if path.extension().and_then(|s| s.to_str())
            == Some("gz")
        {
            Box::new(BufReader::new(
                flate2::read::GzDecoder::new(file),
            ))
        } else {
            Box::new(BufReader::new(file))
        };

    let mut filtered_lines: Vec<String> = Vec::new();

    if let Some(filter) = filter.as_ref() {
        for line_res in file_reader.lines() {
            let line = line_res?;

            if line.is_empty() {
                continue;
            }

            match jmespath::Variable::from_json(&line) {
                Ok(value) => {
                    if let Ok(search_res) =
                        filter.search(value)
                    {
                        if let Some(true) =
                            search_res.as_boolean()
                        {
                            filtered_lines.push(line);
                        }
                    }
                }
                _ => {}
            }
        }
    } else {
        filtered_lines = file_reader
            .lines()
            .flat_map(|res| res)
            .collect();
    }

    Ok(filtered_lines)
}

fn filter_log_files(
    files: ReadDir,
    args: &ParsedArgs,
) -> Vec<PathBuf> {
    let filtered = files
        .filter_map(|item| item.ok())
        .filter_map(|entry| {
            if entry.path().is_file() {
                Some(entry.path())
            } else {
                None
            }
        })
        .filter_map(|file| {
            let path_str = file.to_string_lossy();

            if path_str.ends_with(".log")
                || path_str.ends_with(".log.gz")
            {
                Some(file)
            } else {
                None
            }
        });

    match (&args.from, &args.to) {
        (Some(from), Some(to)) => filtered
            .filter(|file| {
                let created = get_file_created_date(file);

                match created {
                    Some(created) => {
                        created.gt(from) && created.le(to)
                    }
                    None => false,
                }
            })
            .collect(),
        (Some(from), None) => filtered
            .filter(|file| {
                let created = get_file_created_date(file);

                match created {
                    Some(created) => created.gt(from),
                    None => false,
                }
            })
            .collect(),
        (None, Some(to)) => filtered
            .filter(|file| {
                let created = get_file_created_date(file);

                match created {
                    Some(created) => created.le(to),
                    None => false,
                }
            })
            .collect(),

        (None, None) => filtered.collect(),
    }
}

fn get_log_lines(
    args: &ParsedArgs,
    filter: &Option<Expression>,
) -> Vec<String> {
    let all_dirs_result: Vec<Result<Vec<String>, AppError>> = args.dirs.par_iter().map(|dir| {
        let mut files_vec: Vec<PathBuf> = vec![];

        check_dir_existence(dir)?;

        let entries_iter_res = fs::read_dir(dir);

        match entries_iter_res {
            Ok(iter) => files_vec
                .extend(filter_log_files(iter, &args)),
            Err(err) => {
                let _ =
                    write!(stderr(), "{}", err.to_string());
            }
        }

        let available_cpus = get_available_cpus_count()?;
        if available_cpus == 0 {
            return Err(AppError::NoAvailableCpus());
        }

        let (result_sender, result_receiver) = unbounded::<FileProcessingResult>();
        let (task_sender, task_receiver) = unbounded::<PathBuf>();
        let filter_arc = Arc::new(filter.clone());

        scope(|scope| {
            let wg =  WaitGroup::new();

            for _ in 0..available_cpus {
                let wg = wg.clone();
                let thread_task_receiver = task_receiver.clone();
                let thread_result_sender = result_sender.clone();
                let thread_filter = filter_arc.clone();

                scope.spawn(move || {
                    while let Ok(file_path) = thread_task_receiver.recv() {
                        match process_log_file(&file_path, &thread_filter) {
                            Ok(res) => {
                                if !res.is_empty() {
                                    if thread_result_sender.send(res).is_err() {
                                        break;
                                    }
                                }
                            }
                            Err(err) => {
                                report_error(
                                    format!("Error processing file {}", &file_path.to_string_lossy()),
                                    Some(err.to_string()),
                                );
                            }
                        }
                    }

                    drop(wg);
                });
            }

            wg.wait();
        });

        drop(task_sender);



        let dir_files_results: FileProcessingResult =
            result_receiver.iter().flatten().collect();

        Ok(dir_files_results)
    }).collect();

    all_dirs_result
        .iter()
        .flat_map(|dir_result| match dir_result {
            Ok(dirs) => dirs.to_owned(),
            Err(err) => {
                report_error(
                    AppError::CouldNotProcessFile(
                        err.to_string(),
                    )
                    .to_string(),
                    None,
                );
                vec![]
            }
        })
        .collect()
}
