use std::fmt;

//////////////////////////////////////////
/// Spring
//////////////////////////////////////////

#[derive(PartialEq, Clone, Copy, Debug)]
enum Spring {
    OPERATIONAL,
    DAMAGED,
    UNKNOWN
}

use Spring::*;

impl fmt::Display for Spring {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            OPERATIONAL => '.',
            DAMAGED     => '#',
            UNKNOWN     => '?'
        })
    }
}

impl Spring {
    fn from_char(c:char) -> Spring {
        match c {
            '.' => OPERATIONAL,
            '#' => DAMAGED,
            '?' => UNKNOWN,
            _   => unreachable!()
        }
    }
}


#[test]
fn test_display() {
    let spring = Spring::from_char('.');
    assert_eq!(spring, OPERATIONAL);
    assert_eq!(format!("{}", spring), ".");
}

//////////////////////////////////////////
/// Row
//////////////////////////////////////////

type SpringVector = Vec<Spring>;

type DamagedSpringLengths = Vec<u32>;

type DamagedSpringPositions = Vec<u32>;

struct Row {
    springs:SpringVector,
    damaged_spring_lengths:DamagedSpringLengths
}

//////////////////////////////////////////
/// Input parsing
//////////////////////////////////////////

use pest::Parser;
use pest_derive::Parser;
use pest::iterators::Pair;

#[derive(Parser)]
#[grammar = "../grammar/day12.pest"]
struct Day12Parser;

fn build_rows(file_rule:Pair<'_, Rule>) -> Vec<Row> {
    let mut rows:Vec<Row> = Vec::new();

    for row_rule in file_rule.into_inner() {
        match row_rule.as_rule() {
            Rule::row => {
                let mut row = Row{springs: Vec::new(), damaged_spring_lengths: Vec::new()};
                for rule in row_rule.into_inner() {
                    match rule.as_rule() {
                        Rule::springs => {
                            for rule in rule.into_inner() {
                                assert_eq!(rule.as_rule(), Rule::SPRING);
                                assert_eq!(rule.as_str().len(), 1);
                                let c = rule.as_str().chars().next().unwrap();
                                row.springs.push(Spring::from_char(c));
                            }
                        },
                        Rule::damagedLengths => {
                            for rule in rule.into_inner() {
                                assert_eq!(rule.as_rule(), Rule::number);
                                let n = rule.as_str().parse::<u32>().unwrap();
                                row.damaged_spring_lengths.push(n);
                            }
                        },
                        _ => { println!("Unexpected {:?}", rule); }
                    }
                }
                rows.push(row);
            },
            Rule::EOI => (),
            _ => { println!("Unexpected {:?}", row_rule); }
        }
    }
    rows
}

#[test]
fn test_parse() {
    assert_eq!(Day12Parser::parse(Rule::row, "????###??###??##? 11,3").unwrap().as_str(), "????###??###??##? 11,3");
    let input = 
"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";
    let mut parsed = Day12Parser::parse(Rule::file, &input).unwrap();
    let file_rule = parsed.next().unwrap();
    let rows = build_rows(file_rule);
    assert_eq!(rows.len(), 6);
    let row1 = &rows[0];
    assert_eq!(row1.springs, vec![UNKNOWN, UNKNOWN, UNKNOWN, OPERATIONAL, DAMAGED, DAMAGED, DAMAGED]);
    assert_eq!(row1.damaged_spring_lengths, vec![1, 1, 3]);
}
