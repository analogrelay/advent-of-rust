use adventlib::prelude::*;

const DAY: usize = 6;

pub fn run(args: &[String]) -> Result<(), anyhow::Error> {
    Ok(advent_todo(DAY, args))
}
