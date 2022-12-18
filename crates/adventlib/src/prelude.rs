use std::{path, io::{self, Read}, fs, env, str::FromStr, fmt::Debug};

pub fn read_input_lines(day: usize, file_name: &str) -> io::Result<Vec<String>> {
    let mut file = open_day_file(day, file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.lines().map(|s| s.to_string()).collect())
}

pub fn read_input_lines_as<T>(day: usize, file_name: &str) -> Result<Vec<T>, anyhow::Error>
    where T: FromStr, T::Err: Debug {
    let mut file = open_day_file(day, file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut result = Vec::new();
    for line in contents.lines() {
        result.push(line.parse::<T>().unwrap());
    }
    Ok(result)
}

pub fn advent_todo(day: usize, _args: &[String]) -> () {
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