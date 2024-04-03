pub mod debug;
pub mod production;

use super::config::Configs;
use chrono::Local as time;

macro_rules! log_level {
    ($log_level:ident) => {
        fn $log_level(message: &str, show: bool) {
            let logger: Self = LoggerEssentials::open();
            let config = Configs::get();
            let should_log: bool = config.log().on && config.log().kinds.$log_level;

            if should_log {
                let timestamp = time::now().format("%Y-%m-%d %H:%M:%S").to_string();
                let message = format!("[{:?}] {} - {}", stringify!($log_level).to_uppercase(), timestamp, message);
                if config.save() { logger.save(&message); }

                if show {
                    if config.debug() { dbg!(message); }
                    else { println!("{}", message); }
                }
            }
        }
    };
}

pub trait Logger where Self: LoggerEssentials {
    log_level!(info);
    log_level!(trace);
    log_level!(warn);
    log_level!(error);
}

/// Private trait so that the Logger will only be accessible through the Logger trait
trait LoggerEssentials where Self: Sized {
    fn open() -> Self;
    fn save(&self, message: &String);
}
