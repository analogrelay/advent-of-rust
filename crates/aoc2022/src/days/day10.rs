use adventlib::prelude::*;

const DAY: usize = 10;

pub fn run(args: &[String]) -> Result<(), anyhow::Error> {
    Ok(advent_todo(DAY, args))
}