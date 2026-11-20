use std::path::PathBuf;

use clap::Command;
use clap::{CommandFactory, Parser, error::ErrorKind};
use jmespath::Expression;
use time::Date;
use time::macros::format_description;

use crate::AppError;
use crate::utils::{
    get_available_cpus_count, get_filter_exp,
    get_from_date, get_sort_exp, get_to_date,
};

use std::num::NonZeroUsize;

#[derive(Parser, Debug, Clone)]
#[command(name = "Log analyzer")]
#[command(version = "1.0")]
#[command(about = "Analyze JSONL logs with ease", long_about = None)]
pub struct Args {
    /// Directories to scan for log files
    #[arg(short, long, num_args=1..)]
    pub dirs: Vec<PathBuf>,

    /// Date FROM in format YYYY-MM-DD
    #[arg(short, long)]
    pub from: Option<String>,

    /// Date TO in format YYYY-MM-DD
    #[arg(short, long)]
    pub to: Option<String>,

    /// JMESPath expression filter for all lines
    #[arg(long)]
    pub filter: Option<String>,

    /// Line text search
    #[arg(short, long, num_args=1..)]
    pub search: Option<Vec<String>>,

    /// JMESPath expression to sort the final results by
    #[arg(long)]
    pub sort_by: Option<String>,

    /// Num of parallel jobs at a time
    /// (should be greater than 0 and less than thread::available_parallelism)
    /// Default - thread::available_parallelism
    #[arg(short, long)]
    pub jobs: Option<NonZeroUsize>,

    /// Should we print compact version of output
    /// without prettyfying or not
    #[arg(short, long, default_value_t = false)]
    pub compact: bool,

    /// Should we even print result or not (for benchmarking)
    #[arg(short, long, default_value_t = false)]
    pub quiete: bool,
}

pub struct ParsedArgs<'a> {
    pub dirs: Vec<PathBuf>,

    pub from: Option<Date>,

    pub to: Option<Date>,

    pub filter: Option<Expression<'a>>,

    pub sort_by: Option<Expression<'a>>,

    pub jobs: NonZeroUsize,

    pub compact: bool,

    pub quiete: bool,
}

impl Args {
    pub fn init() -> Result<Self, !> {
        let args = Args::parse();
        args.validate()?;

        Ok(args)
    }

    fn validate(&self) -> Result<(), !> {
        let mut cmd = Args::command();

        if let Some(filter) = &self.filter {
            Args::validate_jmespath(
                "filter", filter, &mut cmd,
            )?
        }

        if let Some(sort_expr) = &self.sort_by {
            Args::validate_jmespath(
                "sort expression",
                &sort_expr,
                &mut cmd,
            )?
        }

        if let Some(from) = &self.from {
            Args::validate_date("from", from, &mut cmd)?;
        }

        if let Some(to) = &self.to {
            Args::validate_date("to", to, &mut cmd)?;
        }

        if let Some(jobs) = &self.jobs {
            Args::validate_num_of_jobs(jobs, &mut cmd)?;
        }

        Ok(())
    }

    fn validate_jmespath(
        name: &str,
        expr: &str,
        cmd: &mut Command,
    ) -> Result<(), !> {
        let comp_res = jmespath::compile(expr);

        if let Err(comp_err) = comp_res {
            cmd.error(
                ErrorKind::InvalidValue,
                format!(
                    "{} is not a valid JMESPATH. {}",
                    name, comp_err
                ),
            )
            .exit()
        }

        Ok(())
    }

    fn validate_date(
        name: &str,
        expr: &str,
        cmd: &mut Command,
    ) -> Result<(), !> {
        if let Err(err) = Date::parse(
            expr,
            format_description!("[year]-[month]-[day]"),
        ) {
            cmd.error(
                    ErrorKind::InvalidValue,
                    format!(
                        "Arg {} is invalid.String should be valid ISO_8601 standard. Error: {}",
                        name,
                        err.to_string()
                    ),
                )
                .exit()
        }

        Ok(())
    }

    fn validate_num_of_jobs(
        jobs: &NonZeroUsize,
        cmd: &mut Command,
    ) -> Result<(), !> {
        if let Ok(cpus) =
            std::thread::available_parallelism()
            && cpus < *jobs
        {
            cmd.error(
                ErrorKind::ValueValidation,
                format!(
                    "number of jobs should be less than thread::available_parallelism. Available: {}",
                    cpus
                    )
                ).exit()
        }

        Ok(())
    }

    pub fn parse_args(
        &'_ self
    ) -> Result<ParsedArgs<'_>, AppError> {
        let available_cpus = get_available_cpus_count()?;

        Ok(ParsedArgs {
            dirs: self.dirs.clone(),
            from: get_from_date(self),
            to: get_to_date(self),
            filter: get_filter_exp(self),
            sort_by: get_sort_exp(self),
            jobs: self.jobs.unwrap_or(available_cpus),
            compact: self.compact,
            quiete: self.quiete,
        })
    }
}
