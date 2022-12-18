use std::{str::FromStr, collections::HashSet};

use adventlib::prelude::*;

const DAY: usize = 3;

fn get_priority(c: char) -> usize {
    if c >= 'a' && c <= 'z' {
        c as usize - 'a' as usize + 1
    } else if c >= 'A' && c <= 'Z' {
        c as usize - 'A' as usize + 27
    } else {
        panic!("Invalid item: {}", c);
    }
}

#[derive(Debug)]
pub struct Rucksack(HashSet<char>, HashSet<char>);

impl FromStr for Rucksack {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut left: Vec<_> = s.chars().collect();
        let right = left.split_off(left.len() / 2);
        Ok(Rucksack(left.into_iter().collect(), right.into_iter().collect()))
    }
}

fn part_1(file_name: &str) -> Result<(), anyhow::Error> {
    let rucksacks = read_input_lines_as::<Rucksack>(DAY, file_name)?;
    let result = rucksacks.iter().fold(0, |result, current| {
        // Find the intersecting item
        let duplicate = current.0
            .intersection(&current.1)
            .next()
            .unwrap();
        result + get_priority(*duplicate)
    });
    println!("Part 1 {} result: {}", file_name, result);
    Ok(())
}

fn part_2(file_name: &str) -> Result<(), anyhow::Error> {
    let lines = read_input_lines(DAY, file_name)?;
    let elves = lines.chunks(3);
    let result = elves.fold(0, |result, elf| {
        if elf.len() != 3 {
            panic!("Invalid elf");
        }
        let badge = elf[0].chars().collect::<HashSet<_>>()
            .intersection(&elf[1].chars().collect::<HashSet<_>>())
            .cloned()
            .collect::<HashSet<_>>()
            .intersection(&elf[2].chars().collect::<HashSet<_>>())
            .cloned()
            .next()
            .unwrap();
        result + get_priority(badge)
    });
    println!("Part 2 {} result: {}", file_name, result);
    Ok(())
}

pub fn run(_args: &[String]) -> Result<(), anyhow::Error> {
    part_1("test.txt")?;
    part_1("input.txt")?;
    part_2("test.txt")?;
    part_2("input.txt")?;
    Ok(())
}
