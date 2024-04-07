use crate::logger::{ILogger, LoggerEssentials};

pub(super) struct ProductionLogger;

impl ILogger for ProductionLogger {}

impl LoggerEssentials for ProductionLogger {
    fn open() -> Self {
        todo!()
    }

    fn save(&self, _message: &String) {
        todo!()
    }
}
