use crossbeam::channel::unbounded;
use jmespath::{Expression, Variable};
use rayon::prelude::*;
use std::{
    fs::{self, File, ReadDir},
    io::{BufRead, BufReader},
    path::PathBuf,
    thread::scope,
};

use crate::{
    AppError,
    args::ParsedArgs,
    report_error,
    sort_key::SortKey,
    utils::{check_dir_existence, get_file_created_date},
};

type FileProcessingResult = Vec<(String, Option<SortKey>)>;

pub fn get_log_lines(
    args: &ParsedArgs,
    filter: &Option<Expression>,
    sort: &Option<Expression>,
) -> Vec<String> {
    let mut files: Vec<PathBuf> = args
        .dirs
        .par_iter()
        .flat_map(|dir| {
            let mut files_vec: Vec<PathBuf> = vec![];

            if let Err(err) = check_dir_existence(dir) {
                report_error(err.to_string(), None);
                return vec![];
            }

            let entries_iter_res = fs::read_dir(dir);

            match entries_iter_res {
                Ok(iter) => files_vec
                    .extend(filter_log_files(iter, args)),
                Err(err) => {
                    report_error(err.to_string(), None);
                }
            }

            files_vec
        })
        .collect();

    let jobs = args.jobs.get();
    let mut result = if jobs > 1 {
        process_files_concurrently(
            &mut files, jobs, filter, sort,
        )
    } else {
        process_files(&mut files, filter, sort)
    };

    if sort.is_some() {
        result.sort_by(|a, b| a.1.cmp(&b.1));
    }

    result.into_iter().map(|item| item.0).collect()
}

fn process_files_concurrently(
    files: &mut Vec<PathBuf>,
    jobs: usize,
    filter: &Option<Expression>,
    sort: &Option<Expression>,
) -> FileProcessingResult {
    let (result_sender, result_receiver) =
        unbounded::<FileProcessingResult>();
    let (task_sender, task_receiver) =
        unbounded::<PathBuf>();

    for path in files.drain(..) {
        if task_sender.send(path).is_err() {
            break;
        }
    }
    drop(task_sender);

    scope(|scope| {
        for _ in 0..jobs {
            let thread_task_receiver =
                task_receiver.clone();
            let thread_result_sender =
                result_sender.clone();

            scope.spawn(move || {
                while let Ok(file_path) = thread_task_receiver.recv() {
                    match process_log_file(&file_path, filter, sort) {
                        Ok(res) => {
                            if !res.is_empty() &&  thread_result_sender.send(res).is_err(){
                                break;
                            }
                        }
                        Err(err) => {
                            report_error(
                                format!("Error processing file {}", file_path.to_string_lossy()),
                                Some(err.to_string()),
                            );
                        }
                    }
                }
            });
        }
    });

    drop(result_sender);

    result_receiver.iter().flatten().collect()
}

fn process_files(
    files: &mut Vec<PathBuf>,
    filter: &Option<Expression>,
    sort: &Option<Expression>,
) -> FileProcessingResult {
    let mut result: FileProcessingResult = Vec::new();

    for file_path in files {
        match process_log_file(file_path, filter, sort) {
            Ok(res) => {
                if !res.is_empty() {
                    result.extend(res);
                }
            }
            Err(err) => {
                report_error(
                    format!(
                        "Error processing file {}",
                        file_path.to_string_lossy()
                    ),
                    Some(err.to_string()),
                );
            }
        }
    }

    result
}

fn process_log_file(
    path: &PathBuf,
    filter: &Option<Expression>,
    sort: &Option<Expression>,
) -> Result<FileProcessingResult, AppError> {
    let file = File::open(path)?;
    let mut file_reader: Box<dyn BufRead> =
        if path.extension().and_then(|s| s.to_str())
            == Some("gz")
        {
            Box::new(BufReader::new(
                flate2::read::GzDecoder::new(file),
            ))
        } else {
            Box::new(BufReader::new(file))
        };

    get_file_lines(&mut file_reader, filter, sort)
}

fn get_file_lines(
    reader: &mut Box<dyn BufRead>,
    filter: &Option<Expression>,
    sort: &Option<Expression>,
) -> Result<FileProcessingResult, AppError> {
    reader
        .lines()
        .filter_map::<Result<(String, Option<SortKey>), AppError>, _>(
            |line_res| {
                let line = match line_res {
                    Ok(line) => line,
                    Err(err) => return Some(Err(err.into()))
                };

                if line.is_empty() {
                    return None;
                }

                if let Some(filter) = filter {
                    match get_variable_from_json(&line) {
                        Ok(var) => {
                            if line_passes_filter(
                                &var, filter,
                            ) {
                                if let Some(sort_expr) =
                                    sort
                                {
                                    let sort_key =
                                        match calculate_sort_key(
                                            sort_expr, &var,
                                        ) {
                                            Ok(key) => key,
                                            Err(err) => return Some(Err(err))
                                        };

                                    Some(
                                        Ok(
                                            (
                                                line,
                                                Some(sort_key)
                                            )
                                        )
                                    )
                                } else {
                                        Some(Ok((line, None)))
                                }
                            } else {
                                None
                            }
                        }
                        Err(_) => {
                            None
                        }
                    }
                } else {
                    if let Some(sort_expr) = sort {
                        match get_variable_from_json(&line)
                        {
                            Ok(var) => {
                                let sort_key =
                                        match calculate_sort_key(
                                            sort_expr, &var,
                                        ) {
                                            Ok(key) => key,
                                            Err(err) => return Some(Err(err))
                                        };

                                Some(
                                    Ok(
                                        (
                                            line,
                                            Some(sort_key)
                                        )
                                    )
                                )
                            }
                            Err(_) => {
                                None
                            }
                        }
                    } else {
                        Some(Ok((line, None)))
                    }
                }
            },
        )
        .collect()
}

fn line_passes_filter(
    line: &Variable,
    filter: &Expression,
) -> bool {
    if let Ok(search_res) = filter.search(line)
        && let Some(as_boolean) = search_res.as_boolean()
    {
        return as_boolean;
    }

    false
}

fn get_variable_from_json(
    json: &str
) -> Result<Variable, String> {
    jmespath::Variable::from_json(json)
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

fn calculate_sort_key(
    sort_expr: &Expression,
    line: &Variable,
) -> Result<SortKey, AppError> {
    let val = sort_expr.search(line);

    match val {
        Ok(val) => match SortKey::try_from(val) {
            Ok(key) => Ok(key),
            Err(err) => Err(err),
        },
        Err(err) => Err(err.into()),
    }
}
