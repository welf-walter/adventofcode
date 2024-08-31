use regex::Regex;
use std::cmp;

#[derive(PartialEq)]
#[derive(Debug)]
struct Card
{
    card_index: u32,
    winning_numbers: Vec<u32>,
    numbers_you_have: Vec<u32>
}

fn parse_numbers(input:&str) -> Vec<u32> {
    let mut numbers:Vec<u32> = Vec::new();
    let re = Regex::new(r"\d+").unwrap();
    for number_str in re.find_iter(input).map(|m| m.as_str()) {
        //println!("Parse {}", number_str);
        let number = number_str.parse::<u32>().unwrap();
        numbers.push(number);
    }
    numbers
}

#[test]
fn test_parse_numbers() {
    assert_eq!(parse_numbers("41 48 83 86 17"), [41, 48, 83, 86, 17].to_vec());
    assert_eq!(parse_numbers("83 86  6 86 17"), [83, 86,  6, 86, 17].to_vec());
}

fn parse_line(input:&str) -> Card {
    let re_line = Regex::new(r"Card +(\d+): (( *\d+)*) \| (( *\d+)*)").unwrap();
    if !re_line.is_match(input) {
        panic!("Could not parse: {}", input);
    }
    let caps = re_line.captures(input).unwrap();
    /*println!("  Caps:\n    {}\n    {}\n    {}",
      caps.get(1).unwrap().as_str(),
      caps.get(2).unwrap().as_str(),
      caps.get(4).unwrap().as_str());*/
    let index = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
    let winning_numbers_str = caps.get(2).unwrap().as_str();
    let numbers_you_have_str = caps.get(4).unwrap().as_str();

    Card {
        card_index: index,
        winning_numbers: parse_numbers(winning_numbers_str),
        numbers_you_have: parse_numbers(numbers_you_have_str)
    }
}

fn calculate_card_worth(card:&Card) -> u32 {
    let mut worth = 0;
    for my_number in &card.numbers_you_have {
        if card.winning_numbers.contains(&my_number) {
            worth = cmp::max(1, worth * 2);
        }
    }
    worth
}

#[test]
fn test_example1() {

    let card1 = parse_line("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
    let card2 = parse_line("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19");
    let card3 = parse_line("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1");
    let card4 = parse_line("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83");
    let card5 = parse_line("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36");
    let card6 = parse_line("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11");

    assert_eq!(&card1,
      &Card{
        card_index: 1,
        winning_numbers: [41, 48, 83, 86, 17].to_vec(),
        numbers_you_have: [83, 86, 6, 31, 17, 9, 48, 53].to_vec()
      }
    );
    assert_eq!(&card2,
      &Card{
        card_index: 2,
        winning_numbers: [13, 32, 20, 16, 61].to_vec(),
        numbers_you_have: [61, 30, 68, 82, 17, 32, 24, 19].to_vec()
      }
    );
    assert_eq!(&card3,
      &Card{
        card_index: 3,
        winning_numbers: [1, 21, 53, 59, 44].to_vec(),
        numbers_you_have: [69, 82, 63, 72, 16, 21, 14, 1].to_vec()
      }
    );
    assert_eq!(&card4,
      &Card{
        card_index: 4,
        winning_numbers: [41, 92, 73, 84, 69].to_vec(),
        numbers_you_have: [59, 84, 76, 51, 58, 5, 54, 83].to_vec()
      }
    );
    assert_eq!(&card5,
      &Card{
        card_index: 5,
        winning_numbers: [87, 83, 26, 28, 32].to_vec(),
        numbers_you_have: [88, 30, 70, 12, 93, 22, 82, 36].to_vec()
      }
    );
    assert_eq!(&card6,
      &Card{
        card_index: 6,
        winning_numbers: [31, 18, 13, 56, 72].to_vec(),
        numbers_you_have: [74, 77, 10, 23, 35, 67, 36, 11].to_vec()
      }
    );

    assert_eq!(calculate_card_worth(&card1), 8);
    assert_eq!(calculate_card_worth(&card2), 2);
    assert_eq!(calculate_card_worth(&card3), 2);
    assert_eq!(calculate_card_worth(&card4), 1);
    assert_eq!(calculate_card_worth(&card5), 0);
    assert_eq!(calculate_card_worth(&card6), 0);
}

// -----------------------------------------------------------------------------------

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn part1() {
    let file = File::open("data/day4.input").expect("Could not open data/day4.input");
    let reader = BufReader::new(file);

    let mut cnt_cards = 0;
    let mut worth_cards = 0;

    for line in reader.lines().map(|line| line.unwrap()) {
        let card = parse_line(&line);
        let worth = calculate_card_worth(&card);
        cnt_cards += 1;
        worth_cards += worth;
    }

    println!("Day 4: {} cards with a worth sum of {}", cnt_cards, worth_cards);

}
