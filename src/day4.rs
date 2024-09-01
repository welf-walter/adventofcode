use regex::Regex;
use std::cmp;

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
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

fn count_winning_cards(card:&Card) -> u32 {
  let mut count = 0;
  for my_number in &card.numbers_you_have {
      if card.winning_numbers.contains(&my_number) {
          count += 1;
      }
  }
  count
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

// Multiple instances of one card
#[derive(Clone)]
struct CardInstances {
  card: Card,
  instances: u32

}

#[derive(Clone)]
struct CardDeck {
  // how many instances do I have for these cards?
  cards:Vec<CardInstances>
}

fn parse_deck(lines:Vec<&str>) -> CardDeck {
  let mut card_deck = CardDeck{cards:Vec::new()};
  for line in lines {
    let card = parse_line(&line);
    card_deck.cards.push(CardInstances{card:card, instances:1});
  }
  card_deck
}

fn play_deck(initial_deck:&CardDeck) -> CardDeck {
  let mut deck = initial_deck.clone();
  let card_count = deck.cards.len();
  for current_index in 0..card_count - 1 {
    let current_card_instances = &deck.cards[current_index];
    let current_card = &current_card_instances.card;
    let current_instances = current_card_instances.instances;

    let winning_cards = count_winning_cards(&current_card);
    for win_index in current_index + 1 .. current_index + 1 + winning_cards as usize {
      if win_index < deck.cards.len() {
        deck.cards[win_index as usize].instances += current_instances;
      }
    }
  }
  deck
}

fn count_all_cards_in_deck(deck:&CardDeck) -> u32 {
  let mut count = 0;
  for card_instances in &deck.cards {
    count += card_instances.instances;
  }
  count
}

#[test]
fn test_example2() {

    let lines = [ "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
                  "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
                  "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
                  "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
                  "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
                  "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"].to_vec();

    let initial_deck = parse_deck(lines);

    assert_eq!(initial_deck.cards.len(), 6);
    assert_eq!(initial_deck.cards[2].card.card_index, 3);
    assert_eq!(initial_deck.cards[2].instances, 1);
    assert_eq!(count_all_cards_in_deck(&initial_deck), 6);

    let played_deck = play_deck(&initial_deck);
    assert_eq!(played_deck.cards[0].instances, 1);
    assert_eq!(count_winning_cards(&played_deck.cards[0].card), 4);
    assert_eq!(played_deck.cards[1].instances, 2);
    assert_eq!(count_winning_cards(&played_deck.cards[1].card), 2);
    assert_eq!(played_deck.cards[2].instances, 4);
    assert_eq!(count_winning_cards(&played_deck.cards[2].card), 2);
    assert_eq!(played_deck.cards[3].instances, 8);
    assert_eq!(count_winning_cards(&played_deck.cards[3].card), 1);
    assert_eq!(played_deck.cards[4].instances, 14);
    assert_eq!(count_winning_cards(&played_deck.cards[4].card), 0);
    assert_eq!(played_deck.cards[5].instances, 1);
    assert_eq!(count_winning_cards(&played_deck.cards[5].card), 0);
    assert_eq!(count_all_cards_in_deck(&played_deck), 30);
}

// -----------------------------------------------------------------------------------

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn part1and2() {
    let file = File::open("data/day4.input").expect("Could not open data/day4.input");
    let reader = BufReader::new(file);
    // the following two lines might be a bit too complicated 🙈
    let lines:Vec<String> = reader.lines().map(|line| line.unwrap().to_string()).collect();
    let linesref:Vec<&str> = lines.iter().map(|line| line.as_str() as &str).collect();

    let mut cnt_cards = 0;
    let mut worth_cards = 0;

    let initial_deck = parse_deck(linesref);

    // part 1
    for card in &initial_deck.cards {
      let worth = calculate_card_worth(&card.card);
      cnt_cards += 1;
      worth_cards += worth;
    }

    // part 2
    let played_deck = play_deck(&initial_deck);
    let cnt_all_cards = count_all_cards_in_deck(&played_deck);

    println!("Day 4: {} cards with a worth sum of {}", cnt_cards, worth_cards);
    println!("       In total you have {} cards (original and copied)", cnt_all_cards);

}
