use adventlib::prelude::*;

const DAY: usize = 1;

fn get_calorie_counts(file_name: &str) -> Result<Vec<i32>, anyhow::Error> {
    let lines = read_input_lines(DAY, file_name)?;
    let mut calorie_counts = Vec::new();
    let mut current = 0;
    for line in lines {
        if line.is_empty() {
            calorie_counts.push(current);
            current = 0;
        } else {
            current += line.parse::<i32>()?;
        }
    }
    Ok(calorie_counts)
}

fn part_1(file_name: &str) -> Result<(), anyhow::Error> {
    let counts = get_calorie_counts(file_name)?;
    let max_count = counts.iter().max().ok_or(anyhow::anyhow!("No max"))?;
    println!("Part 1 {} result: {}", file_name, max_count);
    Ok(())
}

fn part_2(file_name: &str) -> Result<(), anyhow::Error> {
    let mut counts = get_calorie_counts(file_name)?;
    counts.sort_unstable();
    if counts.len() < 3 {
        return Err(anyhow::anyhow!("Not enough numbers"));
    }
    let result = counts.pop().unwrap() + counts.pop().unwrap() + counts.pop().unwrap();
    println!("Part 2 {} result: {}", file_name, result);
    Ok(())
}

pub fn run(args: &[String]) -> Result<(), anyhow::Error> {
    part_1("test.txt")?;
    part_1("input.txt")?;
    part_2("test.txt")?;
    part_2("input.txt")?;
    Ok(())
}
