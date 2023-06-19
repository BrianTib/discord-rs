use colored::*;
use chrono::Local;

pub fn log_message(kind: &str, message: &str) {
    let current_time = Local::now().format("%Y-%m-%d %H:%M:%S");
    let colored_message = match kind {
        "warning" => kind.to_uppercase().bold().yellow(),
        "error" => kind.to_uppercase().bold().red(),
        "success" => kind.to_uppercase().bold().green(),
        "event" => kind.to_uppercase().bold().purple(),
        _ => kind.to_uppercase().bold().white(),
    };

    let log_output = format!("[{}] {} - {}", current_time, colored_message, message);
    println!("{}", log_output);
}