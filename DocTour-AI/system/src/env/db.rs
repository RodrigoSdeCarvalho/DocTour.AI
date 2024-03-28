use std::env;
use std::sync::{Mutex, MutexGuard, Once};

use dotenv::{from_path};

use crate::path::{SysPath, join_root, Path};
use crate::env::{Env};

static SINGLETON: Once = Once::new();
static mut DBENV: Option<Mutex<DBEnv>> = None;

pub struct DBEnv {
    host: String,
    port: u16,
    db_name: String,
    user: String,
    pass: String,
}

impl Env for DBEnv {
    fn get<'a>() -> MutexGuard<'a, DBEnv> { // Will be unlocked for as long as the MutexGuard is in the caller's scope
        SINGLETON.call_once(|| {
            unsafe {
                let path: SysPath = join_root!("system", ".env");
                DBENV = Some(Mutex::new(DBEnv::new(path)));
            }
        });

        unsafe {
            DBENV.as_ref()
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
        DBEnv {
            host: env::var("HOST").unwrap(),
            port: env::var("PORT").unwrap()
                .parse::<u16>()
                .unwrap(),
            db_name: env::var("DBNAME").unwrap(),
            user: env::var("DBUSER").unwrap(),
            pass: env::var("PASSWORD").unwrap(),
        }
    }
}

impl DBEnv {
    pub fn open<'a>() -> MutexGuard<'a, DBEnv> {
        Self::get()
    }

    pub fn host(self: &Self) -> String {
        self.host.clone()
    }

    pub fn port(self: &Self) -> u16 {
        self.port.clone()
    }

    pub fn db_name(self: &Self) -> String {
        self.db_name.clone()
    }

    pub fn user(self: &Self) -> String {
        self.user.clone()
    }

    pub fn pass(self: &Self) -> String {
        self.pass.clone()
    }
}
