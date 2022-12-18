use adventlib::prelude::*;

const DAY: usize = 24;

pub fn run(args: &[String]) -> Result<(), anyhow::Error> {
    Ok(advent_todo(DAY, args))
}
