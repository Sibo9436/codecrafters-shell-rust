use std::{env, fs::read_dir, path::PathBuf};

pub(super) fn find_in_path(cmd: &str) -> Option<PathBuf> {
    let path = env::var("PATH").ok()?;

    let path = path.split(':');
    for dir in path {
        if let Ok(entries) = read_dir(dir) {
            for entry in entries {
                let entry = entry.ok()?;
                if entry.file_name() == cmd {
                    return Some(entry.path());
                }
            }
        }
    }
    None
}
