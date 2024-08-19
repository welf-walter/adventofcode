// https://adventofcode.com/2023/day/1

pub fn get_calibration_value(input: &str) -> u32 {

    let decimal = 10;

    let mut first_digit:u32 = 9999;
    let mut have_first_digit = false;
    let mut last_digit:u32 = 9999;

    for c in input.chars() {
        if c.is_digit(decimal) {
          let digit = c.to_digit(decimal).expect("Could not convert digit");
          if !have_first_digit {
            first_digit = digit;
            have_first_digit = true;
          }
          last_digit = digit;
        }
    }

    first_digit * decimal + last_digit
}

#[test]
fn examples() {
    assert_eq!(get_calibration_value("1abc2"), 12);
    assert_eq!(get_calibration_value("pqr3stu8vwx"), 38);
    assert_eq!(get_calibration_value("a1b2c3d4e5f"), 15);
    assert_eq!(get_calibration_value("treb7uchet"), 77);
}

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn part1() {
    //fn main() -> io::Result<()> {
    
    let file = File::open("data/day1.input").expect("Could not open data/day1.input");
    let reader = BufReader::new(file);
    
    let mut sum = 0;
    for line in reader.lines() {
        let value = get_calibration_value(&line.expect("line failure"));
        sum += value;
    }

    println!("Day 1: sum = {}", sum)

}