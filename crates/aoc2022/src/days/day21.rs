use adventlib::prelude::*;

const DAY: usize = 21;

pub fn run(args: &[String]) -> Result<(), anyhow::Error> {
    Ok(advent_todo(DAY, args))
}
