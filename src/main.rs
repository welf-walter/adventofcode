mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

const NUMBER_OF_DAYS : u32 = 8;

use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
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
            8=>day8::part1(),
            otherday=>println!("Unknown day {}", otherday)
        }
    }
}
