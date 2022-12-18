use std::{env, path::PathBuf};

pub mod prelude;

pub fn advent_main<F>(day_runner: F)
    where F: FnOnce(usize, &[String]) -> Result<(), anyhow::Error> {
    let args: Vec<String> = env::args().collect();
    let exe = PathBuf::from(&args[0]);
    let exe_name = exe.file_name().unwrap();

    if args.len() < 2 {
        println!("Usage: {} <day> [args...]", exe_name.to_string_lossy());
        return;
    }

    if let Err(e) = day_runner(args[1].parse().unwrap(), &args[2..]) {
        println!("Day {} failed: {}", args[1], e);
    }
}
