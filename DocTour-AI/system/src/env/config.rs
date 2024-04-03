use std::env;
use std::sync::{Mutex, MutexGuard, Once};

use dotenv::{from_path};

use crate::path::{SysPath, join_root, Path};
use crate::env::{Env};

use benchmark_macro::benchmark;

static SINGLETON: Once = Once::new();
static mut CONFIG: Option<Mutex<Config>> = None;

pub struct Config {
    profile: String
}

impl Env for Config {
    fn get<'a>() -> MutexGuard<'a, Config> { // Will be unlocked for as long as the MutexGuard is in the caller's scope
        SINGLETON.call_once(|| {
            unsafe {
                let path: SysPath = join_root!("system", ".env");
                CONFIG = Some(Mutex::new(Config::new(path)));
            }
        });

        unsafe {
            CONFIG.as_ref()
                .unwrap()
                .lock()
                .unwrap()
        }
    }

    fn new(path: SysPath) -> Self {
        Self::set_env(&path);
        let env: Self = Self::read_env();
        env
    }

    fn set_env(path: &SysPath) -> () {
        from_path(path.as_path()).expect("Failed to read .env file.");
    }

    fn read_env() -> Self {
        Config {
            profile: env::var("PROFILE").unwrap()
        }
    }
}

impl Config {
    #[benchmark]
    pub fn open<'a>() -> MutexGuard<'a, Config> {
        Self::get()
    }

    pub fn profile(self: &Self) -> String {
        self.profile.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let config = Config::open();
        println!("Config profile: {}", config.profile());
        assert_eq!(config.profile(), "DEBUG");
    }
}
