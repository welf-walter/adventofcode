
type Milliseconds = u32;
type Millimeter = u32;

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

use pest::iterators::Pair;

fn build_number_list(number_list_rule:Pair<'_, Rule>) -> Vec<u32> {
    let mut numbers = Vec::new();
    for number in number_list_rule.into_inner() {
        match number.as_rule() {
            Rule::number => {
                let number_value = number.as_str().parse::<u32>().unwrap();
                numbers.push(number_value);
            }
            _ => { println!("Unexpected {}", number); }
        }
    }
    numbers
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

#[cfg(test)]
fn build_example_race_list() -> Vec<Race> {
    let input = [
        "Time:      7  15   30",
        "Distance:  9  40  200",
        ""
    ];
    let concat_input = input.join("\n");
    let mut parsed = Day6Parser::parse(Rule::file, &concat_input).unwrap();
    let file_rule = parsed.next().unwrap();
    build_race_list(file_rule)
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

///// Race logic

impl Race {
    fn calculate_distance(&self, charge:Milliseconds) -> Millimeter {
        assert!(charge <= self.time);
        let travel_time = self.time - charge;
        let speed = charge; // in Millimeter per Millisecond
        speed * travel_time
    }
}
#[test]
fn test_race() {
    let races = build_example_race_list();
    let race1 = &races[0];
    assert_eq!(race1.calculate_distance(0), 0);
    assert_eq!(race1.calculate_distance(1), 6);
    assert_eq!(race1.calculate_distance(2), 10);
    assert_eq!(race1.calculate_distance(3), 12);
    assert_eq!(race1.calculate_distance(4), 12);
    assert_eq!(race1.calculate_distance(5), 10);
    assert_eq!(race1.calculate_distance(6), 6);
    assert_eq!(race1.calculate_distance(7), 0);

}