use loglyzer::{AppError, report_error, run};
use std::process::exit;

fn main() {
    let res = run();

    if let Err(err) = res {
        handle_error_gracefully(err, None);
    }
}

fn handle_error_gracefully(
    err: AppError,
    reason: Option<String>,
) -> ! {
    report_error(err.to_string(), reason);

    exit(1)
}
