mod args;
mod errors;
mod styler;
mod utils;

use std::{
    fs::{self, File, ReadDir},
    io::{prelude::*, stderr, BufReader, Write},
    path::PathBuf,
    process::exit,
    sync::{mpsc, Arc, Mutex},
    thread::{self},
};

use errors::AppError;
use jmespath::Expression;
use serde_json::Value;

use crate::{
    args::{Args, ParsedArgs},
    utils::get_file_created_date,
};
use crate::{
    styler::print_prittified_logs,
    utils::get_available_cpus_count,
};

type FileProcessingResult = Vec<String>;

fn main() {
    let args = Args::init();

    if let Err(err) = args.check() {
        handle_error_gracefully(err.to_string(), None)
    }

    let args = args.parse_args();

    let filter = Arc::new(args.filter.clone());
    let sort_expr = Arc::new(args.sort_by.clone());

    let mut all_dirs_result: Vec<String> = vec![];

    for dir in &args.dirs {
        let mut files_vec: Vec<PathBuf> = vec![];

        let check = fs::exists(&dir);

        match check {
            Ok(res) => {
                if res == false {
                    handle_error_gracefully(
                        format!("Dir {} does not exists", dir.to_string_lossy()),
                        None,
                    );
                }
            }
            Err(err) => handle_error_gracefully(
                format!(
                    "Could not check existance of directory '{}'",
                    dir.to_string_lossy()
                ),
                Some(err.to_string()),
            ),
        }

        let entries_iter_res = fs::read_dir(dir);

        match entries_iter_res {
            Ok(iter) => files_vec
                .extend(filter_log_files(iter, &args)),
            Err(err) => {
                let _ =
                    write!(stderr(), "{}", err.to_string());
            }
        }

        let available_cpus =
            match get_available_cpus_count() {
                Ok(res) => res,
                Err(err) => {
                    handle_error_gracefully(
                        format!(
                    "Could not get available cpus count"
                ),
                        Some(err.to_string()),
                    );
                }
            };

        if available_cpus == 0 {
            handle_error_gracefully(
                String::from(
                    "You have no available cpus left",
                ),
                None,
            );
        }

        let (results_tx, results_rx) =
            mpsc::channel::<FileProcessingResult>();
        let (tx, rx) = mpsc::channel::<PathBuf>();

        let receiver = Arc::new(Mutex::new(rx));

        thread::scope(|s| {
            for _ in 0..available_cpus {
                let thread_receiver = Arc::clone(&receiver);
                let thread_sender = results_tx.clone();
                let thread_filter = Arc::clone(&filter);

                s.spawn(move || {
                    loop {
                        let path_res = thread_receiver.lock().unwrap().recv();

                        match path_res {
                            Ok(path) => {
                                match process_log_file(&path, thread_filter.clone()) {
                                    Ok(res) => {
                                        if !res.is_empty() {
                                            if thread_sender.send(res).is_err() {
                                                break;
                                            }
                                        }
                                    }
                                    Err(err) => {
                                        report_error(
                                            format!("Error processing file {}", &path.to_string_lossy()),
                                            Some(err.to_string()),
                                        );
                                    }
                                }
                            }
                            Err(_) => {
                                break;
                            }
                        }
                    }
                });
            }

            for path in files_vec {
                if tx.send(path).is_err() {
                    break;
                }
            }
            drop(tx);
        });

        drop(results_tx);

        let dir_files_results: FileProcessingResult =
            results_rx.iter().flatten().collect();

        all_dirs_result.extend(dir_files_results);
    }

    if let Some(ref sort_expr) = *sort_expr {
        all_dirs_result.sort_by(|a, b| {
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

    print_prittified_logs(all_dirs_result);
}

fn handle_error_gracefully(
    err: String,
    reason: Option<String>,
) -> ! {
    report_error(err, reason);

    exit(1)
}

fn report_error(
    err: String,
    reason: Option<String>,
) {
    if let Some(reason) = reason {
        let _ = writeln!(
            stderr(),
            "{}. Reason: {}",
            err,
            reason
        );
    } else {
        let _ = writeln!(stderr(), "{}", err);
    }
}

fn process_log_file(
    path: &PathBuf,
    filter: Arc<Option<Expression>>,
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

            if let Ok(value) =
                jmespath::Variable::from_json(&line)
            {
                if let Ok(search_res) = filter.search(value)
                {
                    if let Some(true) =
                        search_res.as_boolean()
                    {
                        filtered_lines.push(line);
                    }
                }
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
