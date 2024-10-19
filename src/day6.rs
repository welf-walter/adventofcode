use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "../grammar/day6.pest"]
struct Day6Parser;

#[test]
fn test_parse() {
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