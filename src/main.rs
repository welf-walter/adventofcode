mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The days to handle
    days: Vec<u32>
}

fn main() {
    let args = Cli::parse();

    let days = if !args.days.is_empty() { args.days } else { (1..6).collect() };

    for day in days {
        match day {
            1=>day1::part1and2(),
            2=>day2::part1and2(),
            3=>day3::part1and2(),
            4=>day4::part1and2(),
            5=>day5::part1and2(),
            otherday=>println!("Unknown day {}", otherday)
        }
    }
}
