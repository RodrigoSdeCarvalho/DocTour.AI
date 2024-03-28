pub mod debug;
pub mod production;

use super::config::Configs;
use chrono::Local as time;

// TODO: Implement Logger trait with a macro
pub trait Logger where Self: Sized {
    fn info(message: &str, show: bool) {
        let logger: Self = Logger::open();
        let config = Configs::get();
        let should_log: bool = config.log().on && config.log().kinds.info;

        if should_log {
            if show { logger.save(message); }
            let timestamp = time::now().format("%Y-%m-%d %H:%M:%S").to_string();
            println!("[INFO] {} - {}", timestamp, message);
        }
    }

    fn trace (message: &str, show: bool) {
        let logger: Self = Logger::open();
        let config = Configs::get();
        let should_log: bool = config.log().on && config.log().kinds.trace;

        if should_log {
            if show { logger.save(message); }
            let timestamp = time::now().format("%Y-%m-%d %H:%M:%S").to_string();
            println!("[TRACE] {} - {}", timestamp, message);
        }
    }

    fn warn(message: &str, show: bool) {
        let logger: Self = Logger::open();
        let config = Configs::get();
        let should_log: bool = config.log().on && config.log().kinds.warn;

        if should_log {
            if show { logger.save(message); }
            let timestamp = time::now().format("%Y-%m-%d %H:%M:%S").to_string();
            println!("[WARNING] {} - {}", timestamp, message);
        }
    }

    fn error (message: &str, show: bool) {
        let logger: Self = Logger::open();

        let config = Configs::get();
        let should_log: bool = config.log().on && config.log().kinds.error;

        if should_log {
            if show { logger.save(message); }
            let timestamp = time::now().format("%Y-%m-%d %H:%M:%S").to_string();
            println!("[ERROR] {} - {}", timestamp, message);
        }
    }

    fn open() -> Self;
    fn save(&self, message: &str);
}

