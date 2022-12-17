use std::{path, io, fs, env};

pub fn advent_todo(day: usize, _args: &[String]) {
    // Mostly just here to make unused-imports warnings go away
    todo!("Implement day {}", day);
}

pub fn day_file_path(day: usize, name: &str) -> path::PathBuf {
    let mut path = data_dir();
    path.push(format!("day{}", day));
    path.push(name);
    path
}

pub fn open_day_file(day: usize, name: &str) -> io::Result<fs::File> {
    let path = day_file_path(day, name);
    fs::File::open(path)
}

pub fn data_dir() -> path::PathBuf {
    fn get_data_dir() -> path::PathBuf {
        if let Ok(dir) = env::var("ADVENT_DATA") {
            path::PathBuf::from(dir)
        } else if let Ok(mut dir) = env::current_dir() {
            dir.push("data");
            dir
        } else {
            panic!("Could not find data directory");
        }
    }

    let dir = get_data_dir();
    if !dir.exists() {
        panic!("Data directory does not exist: {:?}", dir);
    }
    dir
}