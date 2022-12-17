use std::{env, path::PathBuf};

pub mod prelude;

pub fn advent_main<F>(day_runner: F)
    where F: FnOnce(usize, &[String]) -> () {
    let args: Vec<String> = env::args().collect();
    let exe = PathBuf::from(&args[0]);
    let exe_name = exe.file_name().unwrap();

    if args.len() < 2 {
        println!("Usage: {} <day> [args...]", exe_name.to_string_lossy());
        return;
    }

    day_runner(args[1].parse().unwrap(), &args[2..]);
}
