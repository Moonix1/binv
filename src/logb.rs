use env_logger::Builder;
use log::LevelFilter;
use colored::Colorize;

use std::io::Write;

pub struct Log {}

impl Log {
    pub fn init() {
        Builder::new()
            .format(|buf, record| {
                let level = match record.level() {
                    log::Level::Error => format!("{}{}{}", "[".red(), record.level().to_string().red(), "]".red()),
                    log::Level::Warn => format!("{}{}{}", "[".yellow(), record.level().to_string().yellow(), "]".yellow()),
                    log::Level::Info => format!("{}{}{}", "[".blue(), record.level().to_string().blue(), "]".blue()),
                    log::Level::Debug => format!("{}{}{}", "[".yellow(), record.level().to_string().yellow(), "]".blue()),
                    log::Level::Trace => format!("[{}]", record.level().to_string()),
                };

                writeln!(
                    buf,
                    "{}: {}",
                    level,
                    record.args()
                )
            })
            .filter_level(LevelFilter::max())
            .init();
    }
}