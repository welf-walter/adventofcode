
type Milliseconds = u64;
type Millimeter = u64;

#[derive(Debug, PartialEq)]
struct Race {
    time:Milliseconds,
    // current record distance, we must get better
    minimal_distance:Millimeter
}

///// Parser

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "../grammar/day6.pest"]
struct Day6Parser;


#[test]
fn test_parse1() {
    let parse1 = Day6Parser::parse(Rule::number, "42").unwrap().peek().unwrap();
    assert_eq!(parse1.as_rule(), Rule::number);
    assert_eq!(parse1.as_str(), "42");

    assert_eq!(Day6Parser::parse(Rule::number, "6").unwrap().as_str(), "6");

    assert_eq!(Day6Parser::parse(Rule::number_list, "4    15 76").unwrap().as_str(), "4    15 76");

    assert!(Day6Parser::parse(Rule::times, "Time:      7  15   30").is_ok());
    assert!(Day6Parser::parse(Rule::distances, "Distance:  9  40  200").is_ok());

    assert!(Day6Parser::parse(Rule::file,
"Time:      7  15   30
Distance:  9  40  200
").is_ok());

}

#[test]
fn test_parse1_2() {
    let parse1 = Day6Parser::parse(Rule::number_with_spaces, "42 15  3").unwrap().peek().unwrap();
    assert_eq!(parse1.as_rule(), Rule::number_with_spaces);
    assert_eq!(parse1.as_str(), "42 15  3");

    assert_eq!(Day6Parser::parse(Rule::number_with_spaces, "42 15  3").unwrap().as_str(), "42 15  3");

    assert!(Day6Parser::parse(Rule::times2, "Time:      7  15   30").is_ok());
    assert!(Day6Parser::parse(Rule::distances2, "Distance:  9  40  200").is_ok());

    assert!(Day6Parser::parse(Rule::file2,
"Time:      7  15   30
Distance:  9  40  200
").is_ok());

}

use pest::iterators::Pair;

fn build_number_list(number_list_rule:Pair<'_, Rule>) -> Vec<u64> {
    let mut numbers = Vec::new();
    for number in number_list_rule.into_inner() {
        match number.as_rule() {
            Rule::number => {
                let number_value = number.as_str().parse::<u64>().unwrap();
                numbers.push(number_value);
            }
            _ => { println!("Unexpected {}", number); }
        }
    }
    numbers
}

fn build_with_spaces(number_rule:Pair<'_, Rule>) -> u64 {
    number_rule.as_str().replace(" ", "").parse::<u64>().unwrap()
}


fn build_race_list(file_rule:Pair<'_, Rule>) -> Vec<Race> {
    let mut times:Vec<Milliseconds> = Vec::new();
    let mut distances:Vec<Millimeter> = Vec::new();

    for entry in file_rule.into_inner() {
        match entry.as_rule() {
            Rule::times => {
                for rule in entry.into_inner() {
                    match rule.as_rule() {
                        Rule::number_list => { times = build_number_list(rule); },
                        _ => unreachable!(),
                    }
                }
            },
            Rule::distances => {
                for rule in entry.into_inner() {
                    match rule.as_rule() {
                        Rule::number_list => { distances = build_number_list(rule); },
                        _ => unreachable!(),
                    }
                }
            },
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }
    assert_eq!(times.len(), distances.len());

    let mut race_list:Vec<Race> = Vec::new();

    let mut dist_iter = distances.into_iter();
    for time in times {
        let distance = dist_iter.next().unwrap();
        race_list.push(Race { time: time, minimal_distance:distance});
    }

    race_list
}

fn build_race2(file_rule:Pair<'_, Rule>) -> Race {
    let mut time = 0;
    let mut dist = 0;

    for entry in file_rule.into_inner() {
        match entry.as_rule() {
            Rule::times2 => {
                for rule in entry.into_inner() {
                    match rule.as_rule() {
                        Rule::number_with_spaces => { time = build_with_spaces(rule); },
                        _ => unreachable!(),
                    }
                }
            },
            Rule::distances2 => {
                for rule in entry.into_inner() {
                    match rule.as_rule() {
                        Rule::number_with_spaces => { dist = build_with_spaces(rule); },
                        _ => unreachable!(),
                    }
                }
            },
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    Race { time: time, minimal_distance: dist }

}

#[cfg(test)]
fn build_example_race_list() -> Vec<Race> {
    let input = [
        "Time:      7  15   30",
        "Distance:  9  40  200"
    ];
    let concat_input = input.join("\n");
    let mut parsed = Day6Parser::parse(Rule::file, &concat_input).unwrap();
    let file_rule = parsed.next().unwrap();
    build_race_list(file_rule)
}

#[cfg(test)]
fn build_example_race_2() -> Race {
    let input = [
        "Time:      7  15   30",
        "Distance:  9  40  200"
    ];
    let concat_input = input.join("\n");
    let mut parsed = Day6Parser::parse(Rule::file2, &concat_input).unwrap();
    let file_rule = parsed.next().unwrap();
    build_race2(file_rule)
}

#[test]
fn test_parse2() {
    assert_eq!(
        build_number_list(Day6Parser::parse(Rule::number_list, "4    15 76").unwrap().next().unwrap()),
        vec![4, 15, 76]);

    let race_list = build_example_race_list();
    assert_eq!(race_list,
        vec![Race { time: 7,  minimal_distance: 9 },
             Race { time: 15, minimal_distance: 40 },
             Race { time: 30, minimal_distance: 200 }]);

}

#[test]
fn test_parse2_2() {
    assert_eq!(
        build_with_spaces(Day6Parser::parse(Rule::number_with_spaces, "4    15 76").unwrap().next().unwrap()),
        41576);

    let race = build_example_race_2();
    assert_eq!(race,
        Race { time: 71530,  minimal_distance: 940200 });

}

///// Race logic

impl Race {
    fn calculate_distance(&self, charge:Milliseconds) -> Millimeter {
        assert!(charge <= self.time);
        let travel_time = self.time - charge;
        let speed = charge; // in Millimeter per Millisecond
        speed * travel_time
    }

    fn ways_to_win(&self) -> u64 {
        let iter_ways_to_win =
            (0..self.time).filter(
                |charge| self.calculate_distance(*charge) > self.minimal_distance
            );
        iter_ways_to_win.count().try_into().unwrap()
    }
}

fn number_of_ways_to_beat_the_record(races:Vec<Race>) -> u64 {
    let mut number_of_ways_to_beat_the_record = 1;
    for race in races {
        number_of_ways_to_beat_the_record *= race.ways_to_win();
    }
    number_of_ways_to_beat_the_record
}

#[test]
fn test_race() {
    let races = build_example_race_list();
    let race1 = &races[0];
    let race2 = &races[1];
    let race3 = &races[2];

    assert_eq!(race1.calculate_distance(0), 0);
    assert_eq!(race1.calculate_distance(1), 6);
    assert_eq!(race1.calculate_distance(2), 10);
    assert_eq!(race1.calculate_distance(3), 12);
    assert_eq!(race1.calculate_distance(4), 12);
    assert_eq!(race1.calculate_distance(5), 10);
    assert_eq!(race1.calculate_distance(6), 6);
    assert_eq!(race1.calculate_distance(7), 0);

    assert_eq!(race1.ways_to_win(), 4);
    assert_eq!(race2.ways_to_win(), 8);
    assert_eq!(race3.ways_to_win(), 9);

    assert_eq!(number_of_ways_to_beat_the_record(races), 288);

}

#[test]
fn test_race_2() {
    let race = build_example_race_2();
    assert_eq!(race.ways_to_win(), 71503);
}

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
//use std::time::Instant;

pub fn part1and2() {

    let file = File::open("data/day6.input").expect("Could not open data/day6.input");
    let reader = BufReader::new(file);

    let lines:Vec<String> = reader.lines().map( |line| line.unwrap() ).collect();
    let concat_input = lines.join("\n");
    let mut parsed = Day6Parser::parse(Rule::file, &concat_input).unwrap();
    let file_rule = parsed.next().unwrap();
    let races = build_race_list(file_rule);

    println!("Day 6, part 1: Number of ways to beat the record is {}", number_of_ways_to_beat_the_record(races));

    let mut parsed2 = Day6Parser::parse(Rule::file2, &concat_input).unwrap();
    let file_rule2 = parsed2.next().unwrap();
    let race2 = build_race2(file_rule2);

    println!("Day 6, part 2: Number of ways to beat the record is {}", race2.ways_to_win());

}

