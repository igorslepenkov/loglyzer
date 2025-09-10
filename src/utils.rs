use std::{
    fs, io, path::PathBuf, thread::available_parallelism,
};

use jmespath::Expression;
use time::{
    macros::format_description, Date, OffsetDateTime,
};

use crate::args::Args;

pub fn get_available_cpus_count() -> Result<usize, io::Error>
{
    let available_cpus_res = available_parallelism();

    match available_cpus_res {
        Ok(res) => Ok(res.get()),
        Err(err) => Err(err),
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
