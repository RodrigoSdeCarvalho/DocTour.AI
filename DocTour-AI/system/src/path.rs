use std::path;
use std::env;
use std::ffi::OsStr;
use std::sync::{Mutex, MutexGuard, Once};
use std::fmt::Display;

pub type SysPath = path::PathBuf;

static ROOT_NAME: &str = "DocTour-AI";
static mut PATH: Option<Mutex<Path>> = None;
static SINGLETON: Once = Once::new();

pub struct Path {
    root: SysPath,
}

#[macro_export]
macro_rules! join_root {
    ($($arg:expr),*) => {
        Path::join_root(vec![$($arg),*])
    };
}
pub(crate) use join_root; // Exports the macro to the crate

impl Path {

    pub fn get_model<T>(file_name: T) -> SysPath
        where T: Display {
        let model_path = Path::join(&Path::get_models(), file_name.to_string());

        model_path
    }

    pub fn get_raw_chat<T>(file_name: T) -> SysPath
        where T: Display {
        let mut raw_chat_path: SysPath = Path::join(&mut Path::get_assets(), "raw_chats".to_string());
        raw_chat_path = Path::join(&raw_chat_path, file_name.to_string());

        raw_chat_path
    }

    pub fn get_processed_chat<T>(file_name: T) -> SysPath
        where T: Display {
        let mut processed_chat_path: SysPath = Path::join(&mut Path::get_assets(), "processed_chats".to_string());
        processed_chat_path = Path::join(&processed_chat_path, file_name.to_string());

        processed_chat_path
    }

    pub fn get_models() -> SysPath {
        let models_path: SysPath = join_root!("adam", "schemas");

        models_path
    }

    pub fn get_assets() -> SysPath {
        let assets_path: SysPath = join_root!("adam", "assets");

        assets_path
    }

    pub fn join_root(file_folder_names: Vec<&str>) -> SysPath {
        let path: MutexGuard<Path> = Path::get();
        let mut joined_path: SysPath = path.root.clone();

        for file_folder_name in file_folder_names {
            joined_path.push(file_folder_name);
        }

        joined_path
    }

    fn join(path: &SysPath, file_folder_name:String) -> SysPath {
        let mut joined_path: SysPath = path.clone();
        joined_path.push(file_folder_name);

        joined_path
    }

    fn get<'a>() -> MutexGuard<'a, Path> { // Will be unlocked for as long as the MutexGuard is in the caller's scope
        SINGLETON.call_once(|| {
            let root: SysPath = Path::find_root();
            unsafe {
                PATH = Some(Mutex::new(Path { root }));
            }
        });

        unsafe {
            PATH.as_ref()
                .unwrap()
                .lock()
                .unwrap()
        }
    }

    fn find_root() -> SysPath {
        let mut root: SysPath = env::current_exe()
            .unwrap();

        let mut tries: u8 = 0;
        while root.file_name() != Some(OsStr::new(ROOT_NAME)) {
            tries += 1;
            if tries > 10 {
                panic!("Could not find root directory");
            }
            root = root.parent()
                .unwrap()
                .to_path_buf();
        }

        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_root() {
        let executable_path: SysPath = env::current_exe().unwrap();

        let mut root: SysPath = executable_path.clone();
        for _ in 0..4 { // Goes back from target/debug/deps to the root directory
            root = root.parent().unwrap().to_path_buf();
        }

        let found_root: SysPath = Path::find_root();

        assert_eq!(root, found_root);
    }

    #[test]
    fn test_joins() {
        let root: SysPath = Path::find_root();
        let joined_path: SysPath = join_root!("adam", "assets", "raw_chats", "chat.txt");

        assert_eq!(joined_path, Path::join(&root, "adam/assets/raw_chats/chat.txt".to_string()));
    }
}
