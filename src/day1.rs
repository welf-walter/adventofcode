// https://adventofcode.com/2023/day/1

pub fn get_calibration_value1(input: &str) -> u32 {

    const DECIMAL : u32 = 10;

    let mut first_digit:u32 = 9999;
    let mut have_first_digit = false;
    let mut last_digit:u32 = 9999;

    for c in input.chars() {
        if c.is_digit(DECIMAL) {
          let digit = c.to_digit(DECIMAL).expect("Could not convert digit");
          if !have_first_digit {
            first_digit = digit;
            have_first_digit = true;
          }
          last_digit = digit;
        }
    }

    first_digit * DECIMAL + last_digit
}

#[test]
fn examples1() {
    assert_eq!(get_calibration_value1("1abc2"), 12);
    assert_eq!(get_calibration_value1("pqr3stu8vwx"), 38);
    assert_eq!(get_calibration_value1("a1b2c3d4e5f"), 15);
    assert_eq!(get_calibration_value1("treb7uchet"), 77);
}

// get the digit with which the string starts
fn to_digit(input: &str) -> Option<u32> {
    const DECIMAL : u32 = 10;

    let first_character = input.chars().next()?;
    if first_character.is_digit(DECIMAL) {
        return Some(first_character.to_digit(DECIMAL).expect("Could not convert digit"));
    }
    if input.starts_with("one")   { return Some(1); }
    if input.starts_with("two")   { return Some(2); }
    if input.starts_with("three") { return Some(3); }
    if input.starts_with("four")  { return Some(4); }
    if input.starts_with("five")  { return Some(5); }
    if input.starts_with("six")   { return Some(6); }
    if input.starts_with("seven") { return Some(7); }
    if input.starts_with("eight") { return Some(8); }
    if input.starts_with("nine")  { return Some(9); }
    None
}

#[test]
fn test_to_digit() {
    assert_eq!(to_digit("two1nine"), Some(2));
    assert_eq!(to_digit("wo1nine"), None);
    assert_eq!(to_digit("1nine"), Some(1));
    assert_eq!(to_digit(""), None);
}

pub fn get_calibration_value2(input: &str) -> u32 {

    const DECIMAL : u32 = 10;

    let mut first_digit:u32 = 9999;
    let mut have_first_digit = false;
    let mut last_digit:u32 = 9999;

    for offset in 0..input.len() {
        //println!("{}", &input[offset..]);
        match to_digit(&input[offset..]) {
            None => {}
            Some(digit) => {
                if !have_first_digit {
                    first_digit = digit;
                    have_first_digit = true;
                  }
                  last_digit = digit;
                }
        }
    }

    first_digit * DECIMAL + last_digit
}

#[test]
fn examples2() {
    assert_eq!(get_calibration_value2("two1nine"), 29);
    assert_eq!(get_calibration_value2("eightwothree"), 83);
    assert_eq!(get_calibration_value2("abcone2threexyz"), 13);
    assert_eq!(get_calibration_value2("xtwone3four"), 24);
    assert_eq!(get_calibration_value2("4nineeightseven2"), 42);
    assert_eq!(get_calibration_value2("zoneight234"), 14);
    assert_eq!(get_calibration_value2("7pqrstsixteen"), 76);
}

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn part1and2() {
    //fn main() -> io::Result<()> {
    
    let file = File::open("data/day1.input").expect("Could not open data/day1.input");
    let reader = BufReader::new(file);
    
    let mut sum1 = 0;
    let mut sum2 = 0;
    for line in reader.lines() {
        let linetext = &line.expect("line failure");
        let value1 = get_calibration_value1(linetext);
        sum1 += value1;
        let value2 = get_calibration_value2(linetext);
        sum2 += value2;
    }

    println!("Day 1: sum part1 = {}, sum part2 = {}", sum1, sum2)

}

