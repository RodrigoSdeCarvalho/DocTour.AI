use std::fs::OpenOptions;
use std::io::Write;
use chrono::Local as time;

use super::Logger;
use crate::path::{SysPath, Path, join_root};

struct DebugLogger {
    folder: SysPath,
    file_name: String,
}

impl Logger for DebugLogger {
    fn open() -> Self {
        let timestamp = time::now().format("%Y-%m-%d_%H-%M-%S").to_string();
        let folder = join_root!("system", "logs");

        return DebugLogger {
            folder,
            file_name: format!("log_{}.txt", timestamp),
        };
    }

    fn save(&self, message: &str) {
        let path = self.folder.join(&self.file_name);

        let mut file = OpenOptions::new()
            .append(true)
            .open(&path)
            .unwrap();

        file.write_all(message.as_bytes()).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logger() {
        DebugLogger::info("Test info message", true);
        DebugLogger::trace("Test trace message", true);
        DebugLogger::warn("Test warning message", true);
        DebugLogger::error("Test error message", true);
    }
}
