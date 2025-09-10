use std::path::PathBuf;

use clap::Parser;
use jmespath::Expression;
use time::macros::format_description;
use time::Date;

use crate::errors::AppError;
use crate::utils::{
    get_filter_exp, get_from_date, get_sort_exp,
    get_to_date,
};

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
}

pub struct ParsedArgs<'a> {
    pub dirs: Vec<PathBuf>,

    pub from: Option<Date>,

    pub to: Option<Date>,

    pub filter: Option<Expression<'a>>,

    pub sort_by: Option<Expression<'a>>,
}

impl Args {
    pub fn init() -> Self {
        Args::parse()
    }

    pub fn check(&self) -> Result<(), AppError> {
        if let Some(filter) = &self.filter {
            jmespath::compile(filter)?;
        }

        if let Some(sort_expr) = &self.sort_by {
            jmespath::compile(sort_expr)?;
        }

        if let Some(from) = &self.from {
            println!("FROM: {}", from);
            Date::parse(from, format_description!("[year]-[month]-[day]")).map_err(|err| AppError::DateCheckError(format!("Could not parse 'from' value. String should be valid ISO_8601 standard. Error: {}", err)))?;
        }

        if let Some(to) = &self.to {
            Date::parse(to, format_description!("[year]-[month]-[day]")).map_err(|_| AppError::DateCheckError(String::from("Could not parse 'to' value. String should be valid ISO_8601 standard")))?;
        }

        Ok(())
    }

    pub fn parse_args(&'_ self) -> ParsedArgs<'_> {
        ParsedArgs {
            dirs: self.dirs.clone(),
            from: get_from_date(self),
            to: get_to_date(self),
            filter: get_filter_exp(self),
            sort_by: get_sort_exp(self),
        }
    }
}
