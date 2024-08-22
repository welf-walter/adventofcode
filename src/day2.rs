use regex::Regex;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
struct Set {
    red: u32,
    blue: u32,
    green: u32
}

#[test]
fn test_regex() {
    assert!(Regex::new(r"\d+").unwrap().is_match("46"));
    assert!(Regex::new(r"(blue|red|green)").unwrap().is_match("red"));
    assert!(Regex::new(r"(\d+) (blue|red|green)").unwrap().is_match("6 green"));
    assert!(Regex::new(r"((\d+) (blue|red|green),?)+").unwrap().is_match("7 blue, 99 green"));
}

fn parse_set(input:&str) -> Set {
    let re = Regex::new(r"((\d+) (blue|red|green),?)+").unwrap();
    if !re.is_match(input) {
        panic!("Could not parse: {}", input);
    }
    let mut set:Set = Set{red:0,blue:0,green:0};
    for (_, [_, countstr, color]) in re.captures_iter(input).map(|c| c.extract()) {
        let count = countstr.parse::<u32>().unwrap();
        //println!("{} {}", count, color);
        if color == "red" {
            assert!(set.red == 0);
            set.red = count;
        } else if color == "blue" {
            assert!(set.blue == 0);
            set.blue = count;
        } else if color == "green" {
            assert!(set.green == 0);
            set.green = count;
        } else {
            panic!("Unknown color {}", color);
        }
    }

    set
}

#[test]
fn test_parse_set() {
    assert_eq!(parse_set("3 blue, 4 red"), Set{red:4, green:0, blue: 3});
    assert_eq!(parse_set("1 red, 2 green"), Set{red:1, green:2, blue: 0});
    assert_eq!(parse_set("2 green"), Set{red:0, green:2, blue: 0});
}

#[test]
fn test_regex2() {
    assert!(Regex::new(r"(\d|\w)+").unwrap().is_match("46 blue"));
    assert!(Regex::new(r"(\d|\w|,| )+").unwrap().is_match("46 blue, 88 green"));
    assert!(Regex::new(r"(\d|\w|,| )+").unwrap().is_match("46 blue, 88 green; 12 red"));
    assert!(Regex::new(r"((\d|\w|,| )+;?)+").unwrap().is_match("46 blue, 88 green; 12 red"));
    assert!(Regex::new(r"^(;?(\d|\w|,| )+)+$").unwrap().is_match("46 blue, 88 green; 12 red"));
}

fn parse_sets(input:&str) -> Vec<Set> {
    let re = Regex::new(r"((\d|\w|,| )+)").unwrap();
    if !re.is_match(input) {
        panic!("Could not parse: {}", input);
    }
    let mut sets=Vec::new();
    for setstr in re.find_iter(input).map(|m| m.as_str()) {
    //for (_, [_, setstr, _]) in re.captures_iter(input).map(|c| c.extract()) {
        //println!("  Set: {}", setstr);
        sets.push(parse_set(setstr));
    }

    sets
}

#[test]
fn test_parse_sets() {
    assert_eq!(parse_sets("3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
               [Set{red:4,green:0,blue:3},Set{red:1,green:2,blue:6},Set{red:0,green:2,blue:0}]);
}


#[derive(Debug)]
#[derive(PartialEq)]
struct Game {
    index: u32,
    sets:Vec<Set>
}


fn parse_game(input:&str) -> Game {
    //println!("Input: {}", input);
    let re = Regex::new(r"Game (\d+): (.+)").unwrap();
    if !re.is_match(input) {
        panic!("Could not parse: {}", input);
    }
    let caps = re.captures(input).unwrap();
    //println!("  {} - {}", caps.get(1).unwrap().as_str(), caps.get(2).unwrap().as_str());
    let index = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
    let setsstr = caps.get(2).unwrap().as_str();
    let sets = parse_sets(setsstr);
    Game{index:index, sets:sets}
}

fn is_possible(game:&Game) -> bool {
    let bag=Set{red:12, green:13, blue:14};
    for set in &game.sets {
        if set.red   > bag.red   ||
           set.green > bag.green ||
           set.blue  > bag.blue {
            return false;
        }
    }

    true
}

#[test]
fn examples1() {
    let game1 = parse_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
    assert_eq!(game1, Game{index:1, sets:[Set{red:4, green:0, blue: 3}, Set{red:1,green:2,blue:6}, Set{red:0,green:2,blue:0}].to_vec()});
    assert_eq!(is_possible(&game1),  true);
    let game2 = parse_game("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue");
    assert_eq!(game2, Game{index:2, sets:[Set{red:0, green:2, blue: 1}, Set{red:1,green:3,blue:4}, Set{red:0,green:1,blue:1}].to_vec()});
    assert_eq!(is_possible(&game2),  true);
    let game3 = parse_game("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red");
    assert_eq!(game3, Game{index:3, sets:[Set{red:20, green:8, blue: 6}, Set{red:4,green:13,blue:5}, Set{red:1,green:5,blue:0}].to_vec()});
    assert_eq!(is_possible(&game3),  false);
    let game4 = parse_game("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red");
    assert_eq!(game4, Game{index:4, sets:[Set{red:3, green:1, blue: 6}, Set{red:6,green:3,blue:0}, Set{red:14,green:3,blue:15}].to_vec()});
    assert_eq!(is_possible(&game4),  false);
    let game5 = parse_game("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
    assert_eq!(game5, Game{index:5, sets:[Set{red:6, green:3, blue: 1}, Set{red:1,green:2,blue:2}].to_vec()});
    assert_eq!(is_possible(&game5),  true);
}

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn part1() {
    let file = File::open("data/day2.input").expect("Could not open data/day2.input");
    let reader = BufReader::new(file);

    let mut sum_of_indices = 0;
    let mut cnt_of_possible_games = 0;
    let mut cnt_of_games = 0;
    for line in reader.lines() {
        let linetext = &line.expect("line failure");
        let game = parse_game(linetext);
        if is_possible(&game) {
            sum_of_indices += game.index;
            cnt_of_possible_games += 1;
        }
        cnt_of_games += 1;
    }

    println!("Day 2: {} of {} games possible. Sum of indices = {}", cnt_of_possible_games, cnt_of_games, sum_of_indices);
}
