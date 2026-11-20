mod args;
mod errors;
mod file_processing;
mod sort_key;
mod styler;
mod utils;

pub use errors::AppError;

use crate::styler::print_prittified_logs;
use crate::{args::Args, file_processing::get_log_lines};

pub use crate::utils::report_error;

pub fn run() -> Result<(), errors::AppError> {
    let args = Args::init().unwrap();

    let args = args.parse_args()?;

    let all_lines =
        get_log_lines(&args, &args.filter, &args.sort_by);

    if !args.quiete {
        if args.compact {
            for line in all_lines {
                println!("{}", line);
            }
        } else {
            print_prittified_logs(all_lines);
        }
    }

    Ok(())
}
