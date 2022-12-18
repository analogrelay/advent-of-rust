use adventlib::prelude::*;

const DAY: usize = 9;

pub fn run(args: &[String]) -> Result<(), anyhow::Error> {
    Ok(advent_todo(DAY, args))
}
