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

const NUMBER_OF_DAYS : u32 = 11;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// The days to handle
    days: Vec<u32>
}

fn main() {
    let args = Cli::parse();

    let days = if !args.days.is_empty() { args.days } else { (1..NUMBER_OF_DAYS+1).collect() };

    for day in days {
        match day {
            1=>day1::part1and2(),
            2=>day2::part1and2(),
            3=>day3::part1and2(),
            4=>day4::part1and2(),
            5=>day5::part1and2(),
            6=>day6::part1and2(),
            7=>day7::part1and2(),
            8=>day8::part1and2(),
            9=>day9::part1and2(),
            10=>day10::part1and2(),
            11=>day11::part1and2(),
            otherday=>println!("Unknown day {}", otherday)
        }
    }
}
