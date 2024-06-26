mod debug;
mod production;

use super::config::{Configs, Profile};
use chrono::Local as time;

use debug::DebugLogger;
use production::ProductionLogger;

/// Generates the trace, info, warn and error methods that'll be public available
/// through the Logger interface. Depending on the build's profile, a different Logger
/// implementation will be called.
macro_rules! log {
    ($log_level:ident) => {
        pub fn $log_level<T: AsRef<str>>(message: T, show: bool) {
            let profile = {
                let config = Configs::open().lock().unwrap();
                config.profile().clone()
            };

            match profile {
                Profile::DEBUG => DebugLogger::$log_level(message, show),
                Profile::PRODUCTION => ProductionLogger::$log_level(message, show)
            };
        }
    };
}

pub struct Logger;

impl Logger {
    log!(info);
    log!(trace);
    log!(warn);
    log!(error);
}

/// Generates the trace, info, warn & error methods for the ILogger trait.
macro_rules! log_level {
    ($log_level:ident) => {
        fn $log_level<T: AsRef<str>>(message: T, show: bool) {
            let logger: Self = LoggerEssentials::open();

            let (should_log, save, debug): (bool, bool, bool) = {
                let config = Configs::open().lock().unwrap();

                let should_log = config.log().on && config.log().kinds.$log_level;
                let save = config.save();
                let debug = config.debug();

                (should_log, save, debug)
            };

            if should_log {
                let timestamp = time::now().format("%Y-%m-%d %H:%M:%S").to_string();
                let message = format!("[{:?}] {} - {}", stringify!($log_level).to_uppercase(), timestamp, message.as_ref());
                if save { logger.save(&message); }

                if show {
                    if debug { dbg!(message); }
                    else { println!("{}", message); }
                }
            }
        }
    };
}

trait ILogger where Self: LoggerEssentials {
    log_level!(info);
    log_level!(trace);
    log_level!(warn);
    log_level!(error);
}

trait LoggerEssentials where Self: Sized {
    fn open() -> Self;
    fn save(&self, message: &String);
}

#[cfg(test)]
mod tests {
    use super::*;

    // Make sure log is on and save is true (adjust the system/configs.json file)
    #[test]
    fn test_logger() {
        Logger::info("Test info message", true);
        Logger::trace("Test trace message".to_string(), true);
        Logger::warn(&"Test warning message".to_string(), true);
        let test: String = String::from("Test error message");
        Logger::error(test, true);
    }
}
