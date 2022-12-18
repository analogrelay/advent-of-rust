use std::str::FromStr;

use adventlib::prelude::*;

const DAY: usize = 2;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    pub fn score(self) -> usize {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Outcome {
    pub fn score(self) -> usize {
        match self {
            Outcome::Loss => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

struct Step(Move, Move, Outcome);

impl FromStr for Step {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splat = s.split_whitespace();
        let first = match splat.next().unwrap() {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => return Err(anyhow::anyhow!("Invalid move")),
        };
        let (second, outcome) = match splat.next().unwrap() {
            "X" => (Move::Rock, Outcome::Loss),
            "Y" => (Move::Paper, Outcome::Draw),
            "Z" => (Move::Scissors, Outcome::Win),
            _ => return Err(anyhow::anyhow!("Invalid move")),
        };
        Ok(Step(first, second, outcome))
    }
}

fn part_1(file_name: &str) -> Result<(), anyhow::Error> {
    let steps = read_input_lines_as::<Step>(DAY, file_name)?;
    let result = steps.iter().fold(0, |score, current| {
        let outcome = match current {
            Step(Move::Rock, Move::Rock, _) => Outcome::Draw,
            Step(Move::Rock, Move::Paper, _) => Outcome::Win,
            Step(Move::Rock, Move::Scissors, _) => Outcome::Loss,
            Step(Move::Paper, Move::Rock, _) => Outcome::Loss,
            Step(Move::Paper, Move::Paper, _) => Outcome::Draw,
            Step(Move::Paper, Move::Scissors, _) => Outcome::Win,
            Step(Move::Scissors, Move::Rock, _) => Outcome::Win,
            Step(Move::Scissors, Move::Paper, _) => Outcome::Loss,
            Step(Move::Scissors, Move::Scissors, _) => Outcome::Draw,
        };
        score + outcome.score() + current.1.score()
    });
    println!("Part 1 {} result: {}", file_name, result);
    Ok(())
}

fn part_2(file_name: &str) -> Result<(), anyhow::Error> {
    let steps = read_input_lines_as::<Step>(DAY, file_name)?;
    let result = steps.iter().fold(0, |score, current| {
        let my_move = match current {
            Step(Move::Rock, _, Outcome::Loss) => Move::Scissors,
            Step(Move::Rock, _, Outcome::Draw) => Move::Rock,
            Step(Move::Rock, _, Outcome::Win) => Move::Paper,
            Step(Move::Paper, _, Outcome::Loss) => Move::Rock,
            Step(Move::Paper, _, Outcome::Draw) => Move::Paper,
            Step(Move::Paper, _, Outcome::Win) => Move::Scissors,
            Step(Move::Scissors, _, Outcome::Loss) => Move::Paper,
            Step(Move::Scissors, _, Outcome::Draw) => Move::Scissors,
            Step(Move::Scissors, _, Outcome::Win) => Move::Rock,
        };
        score + current.2.score() + my_move.score()
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
