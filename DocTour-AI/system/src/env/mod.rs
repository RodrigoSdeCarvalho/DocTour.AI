pub mod config;
pub mod db;

use crate::path::{SysPath};
use std::sync::{MutexGuard};

trait Env {
    fn get<'a>() -> MutexGuard<'a, Self>;
    fn new(path: SysPath) -> Self;
    fn set_env(path: &SysPath) -> ();
    fn read_env() -> Self;
}

// TODO: Implement a macro to create specific env structs
