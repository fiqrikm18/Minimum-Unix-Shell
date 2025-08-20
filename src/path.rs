use std::path::PathBuf;

pub fn retrieve_os_path() -> Vec<PathBuf> {
    let paths = std::env::var("PATH");
    let os_path: Vec<PathBuf> = match paths {
        Ok(path) => std::env::split_paths(&path).collect(),
        Err(err) => {
            println!("{}", err);
            Vec::new()
        }
    };

    os_path
}
