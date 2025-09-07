use std::{io, thread::available_parallelism};

use jmespath::Expression;

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
