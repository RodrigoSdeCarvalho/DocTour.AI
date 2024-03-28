use std::sync::{Mutex, MutexGuard, Once};
use serde::{Deserialize, Serialize};

use crate::env::{config::Config as Env};
use crate::join_root;
use crate::path::{Path, SysPath};

static SINGLETON: Once = Once::new();
static mut CONFIGS: Option<Mutex<Configs>> = None;

#[derive(Serialize, Deserialize, Debug)]
pub struct Kinds {
    pub trace: bool,
    pub info: bool,
    pub warn: bool,
    pub error: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Log {
    pub on: bool,
    pub kinds: Kinds,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Configs {
    log: Log,
    profile: Option<String>,
}

impl Configs {
    pub fn get<'a>() -> MutexGuard<'a, Configs> { // Will be unlocked for as long as the MutexGuard is in the caller's scope
        SINGLETON.call_once(|| {
            unsafe {
                CONFIGS = Some(Mutex::new(Configs::new()));
            }
        });

        unsafe {
            CONFIGS.as_ref()
                .unwrap()
                .lock()
                .unwrap()
        }
    }

    fn new() -> Configs {
        let config: SysPath= join_root!("system", "configs.json");
        let content: String = std::fs::read_to_string(config).unwrap();
        let config: Configs = serde_json::from_str(&content).unwrap();
        let profile: String = Env::open().profile();

        Configs {
            profile: Some(profile),
            ..config
        }
    }

    pub fn log(&self) -> &Log {
        &self.log
    }

    pub fn profile(&self) -> &String {
        self.profile.as_ref().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::config::Configs;

    #[test]
    fn test_new() {
        let config = Configs::get();
        println!("{:?}", config.profile());
        println!("{:?}", config.log());
    }
}
