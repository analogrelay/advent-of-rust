mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

pub fn run_day(day: usize, args: &[String]) {
    match day {
        1 => day1::run(args),
        2 => day2::run(args),
        3 => day3::run(args),
        4 => day4::run(args),
        5 => day5::run(args),
        6 => day6::run(args),
        7 => day7::run(args),
        8 => day8::run(args),
        9 => day9::run(args),
        10 => day10::run(args),
        11 => day11::run(args),
        12 => day12::run(args),
        13 => day13::run(args),
        14 => day14::run(args),
        15 => day15::run(args),
        16 => day16::run(args),
        17 => day17::run(args),
        18 => day18::run(args),
        19 => day19::run(args),
        20 => day20::run(args),
        21 => day21::run(args),
        22 => day22::run(args),
        23 => day23::run(args),
        24 => day24::run(args),
        25 => day25::run(args),
        _ => panic!("Unknown day: {}", day),
    }
}
