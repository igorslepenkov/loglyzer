use std::{
    fs,
    io::{Write, stderr},
    num::NonZeroUsize,
    path::PathBuf,
    thread::available_parallelism,
};

use jmespath::Expression;
use time::{
    Date, OffsetDateTime, macros::format_description,
};

use crate::{args::Args, errors::AppError};

pub fn get_available_cpus_count()
-> Result<NonZeroUsize, AppError> {
    let available_cpus_res = available_parallelism();

    match available_cpus_res {
        Ok(res) => Ok(res),
        Err(err) => {
            Err(AppError::CouldNotReadAvailableCpusLength(
                err.to_string(),
            ))
        }
    }
}

pub fn get_filter_exp(
    args: &Args
) -> Option<Expression<'_>> {
    match &args.filter {
        Some(string) => {
            Some(jmespath::compile(string).unwrap())
        }
        None => None,
    }
}

pub fn get_sort_exp(args: &Args) -> Option<Expression<'_>> {
    match &args.sort_by {
        Some(string) => {
            Some(jmespath::compile(string).unwrap())
        }
        None => None,
    }
}

pub fn get_to_date(args: &Args) -> Option<Date> {
    match &args.to {
        Some(date_string) => Some(
            Date::parse(
                date_string,
                format_description!("[year]-[month]-[day]"),
            )
            .unwrap(),
        ),
        None => None,
    }
}

pub fn get_from_date(args: &Args) -> Option<Date> {
    match &args.from {
        Some(date_string) => Some(
            Date::parse(
                date_string,
                format_description!("[year]-[month]-[day]"),
            )
            .unwrap(),
        ),
        None => None,
    }
}

pub fn get_file_created_date(
    file: &PathBuf
) -> Option<Date> {
    let file_metadata_res = fs::metadata(file);

    match file_metadata_res {
        Ok(metadata) => {
            let created_res = metadata.created();

            match created_res {
                Ok(created_system_time) => Some(
                    OffsetDateTime::from(
                        created_system_time,
                    )
                    .date(),
                ),
                Err(_) => None,
            }
        }
        Err(_) => None,
    }
}

pub fn check_dir_existence(
    dir: &PathBuf
) -> Result<(), AppError> {
    let check = fs::exists(&dir);

    match check {
            Ok(res) => {
                if res == false {
                    return Err(AppError::DirectoryNotFound(dir.to_string_lossy().to_string()))
                }

                Ok(())
            }
            Err(err) => Err(AppError::DirectoryExistenceCouldNotBeCheckedDueToError(dir.to_string_lossy().to_string(), err.to_string()))
        }
}

pub fn report_error(
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
