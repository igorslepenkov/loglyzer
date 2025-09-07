use colored_json::{
    Color, ColoredFormatter, PrettyFormatter, Styler,
};
use serde_json::Value;

pub fn print_prittified_logs(logs: Vec<String>) {
    let pretty_formatter = PrettyFormatter::new();

    let styler = Styler {
        key: Color::Green.foreground(),
        string_value: Color::Blue.bold(),
        ..Default::default()
    };

    let f = ColoredFormatter::with_styler(
        pretty_formatter,
        styler,
    );

    for line in logs {
        match serde_json::from_str::<Value>(&line) {
            Ok(parsed_value) => {
                match f
                    .clone()
                    .to_colored_json_auto(&parsed_value)
                {
                    Ok(value) => println!("{}", value),
                    Err(_) => println!("{}", line),
                }
            }
            Err(_) => {
                println!("{}", line);
            }
        }
    }
}
