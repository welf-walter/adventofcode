//////////////////////////////////////////
/// Part of Day
//////////////////////////////////////////

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Part {
    Part1,
    Part2
}

use Part::Part1;
use Part::Part2;

//////////////////////////////////////////
/// Card
//////////////////////////////////////////

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    _9,
    _8,
    _7,
    _6,
    _5,
    _4,
    _3,
    _2,
    Joker // for part 2
}

impl Card {
    fn get_value(&self) -> u32 {
        match self {
            Card::A => 20,
            Card::K => 13,
            Card::Q => 12,
            Card::J => 11,
            Card::T => 10,
            Card::_9 => 9,
            Card::_8 => 8,
            Card::_7 => 7,
            Card::_6 => 6,
            Card::_5 => 5,
            Card::_4 => 4,
            Card::_3 => 3,
            Card::_2 => 2,
            Card::Joker => 1
        }
    }

    fn from_char(c:char, part:Part) -> Card {
        match c {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => match part { Part1 => Card::J, Part2 => Card::Joker },
            'T' => Card::T,
            '9' => Card::_9,
            '8' => Card::_8,
            '7' => Card::_7,
            '6' => Card::_6,
            '5' => Card::_5,
            '4' => Card::_4,
            '3' => Card::_3,
            '2' => Card::_2,
            _ => unreachable!()
        }
    }

    fn to_char(&self) -> char {
        match self {
            Card::A => 'A',
            Card::K => 'K',
            Card::Q => 'Q',
            Card::J => 'J',
            Card::T => 'T',
            Card::_9 => '9',
            Card::_8 => '8',
            Card::_7 => '7',
            Card::_6 => '6',
            Card::_5 => '5',
            Card::_4 => '4',
            Card::_3 => '3',
            Card::_2 => '2',
            Card::Joker => '*'
        }
    }

}

use std::cmp::Ordering;

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.get_value().partial_cmp(&other.get_value())
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_value().cmp(&other.get_value())
    }
}

#[test]
fn test_card() {
    assert!(Card::_4 < Card::_5);
    assert_eq!(Card::from_char('K', Part1), Card::K);
    assert_eq!(Card::from_char('5', Part1), Card::_5);
    assert_eq!(Card::from_char('J', Part1), Card::J);
    assert_eq!(Card::from_char('J', Part2), Card::Joker);
    assert_eq!(Card::_5.to_char(), '5');
}

//////////////////////////////////////////
/// Hand
//////////////////////////////////////////

#[derive(Debug, PartialEq, Eq, Clone)]
struct Hand {
    cards:[Card;5]
}

impl Hand {
    fn from_str(s:&str, part:Part) -> Hand {
        assert!(s.len()==5);
        let mut c5 = s.chars();
        Hand{cards:[Card::from_char(c5.next().unwrap(), part),
                    Card::from_char(c5.next().unwrap(), part),
                    Card::from_char(c5.next().unwrap(), part),
                    Card::from_char(c5.next().unwrap(), part),
                    Card::from_char(c5.next().unwrap(), part)]}
    }

    fn to_char5(&self) -> [char;5] {
        [
            self.cards[0].to_char(),
            self.cards[1].to_char(),
            self.cards[2].to_char(),
            self.cards[3].to_char(),
            self.cards[4].to_char()
        ]
    }

    fn to_string(&self) -> String {
        self.to_char5().iter().collect()
    }
}

#[test]
fn test_hand() {
    assert_eq!(Hand::from_str("32TJK", Part1), Hand{cards:[Card::_3, Card::_2, Card::T, Card::J, Card::K]});
    assert_eq!(Hand::from_str("32TJK", Part2), Hand{cards:[Card::_3, Card::_2, Card::T, Card::Joker, Card::K]});
    assert_eq!(Hand::from_str("32T3K", Part1).to_char5(), ['3', '2', 'T', '3', 'K']);
    assert_eq!(Hand::from_str("32T3K", Part1).to_string(), "32T3K");
    assert_eq!(Hand::from_str("3JTJK", Part2).to_string(), "3*T*K");
}

//////////////////////////////////////////
/// Hand Type
//////////////////////////////////////////

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind
}

impl HandType {

    fn count_joker(cards:&[Card;5]) -> u32 {
        cards.iter().filter(|card| **card == Card::Joker).count().try_into().unwrap()
    }

    fn count_n_of_a_kind<const N:usize>(sorted_cards:&[Card;5]) -> u32 {
        const OUTPUT:bool = false; // for error analysis
        let mut count = 0;
        let mut current_card = sorted_cards[0];
        if OUTPUT { print!("{}", current_card.to_char()); }
        let mut equal_counter = 1;
        for index in 1..5 {
            if current_card == sorted_cards[index] {
                if OUTPUT { print!("={}", sorted_cards[index].to_char()); };
                equal_counter += 1;
            } else {
                if OUTPUT { print!("|{}", sorted_cards[index].to_char()); };
                if equal_counter == N && current_card != Card::Joker {
                    count += 1;
                }
                current_card = sorted_cards[index];
                equal_counter = 1;
            }
        }
        if equal_counter == N && current_card != Card::Joker {
            count += 1;
        }
        if OUTPUT { println!("  {} times {} of a kind", count, N); };
        count
    }

    fn of(hand:&Hand) -> Self {
        let mut cards = hand.cards.clone();
        cards.sort();

        let count5 = Self::count_n_of_a_kind::<5>(&cards);
        let count4 = Self::count_n_of_a_kind::<4>(&cards);
        let count3 = Self::count_n_of_a_kind::<3>(&cards);
        let count2 = Self::count_n_of_a_kind::<2>(&cards);

        match Self::count_joker(&cards) {
            5 => return HandType::FiveOfAKind,
            4 => return HandType::FiveOfAKind,
            3 => {
                if count2 == 1 { return HandType::FiveOfAKind; };
                return HandType::FourOfAKind;
            },
            2 => {
                if count3 == 1 { return HandType::FiveOfAKind; };
                if count2 == 1 { return HandType::FourOfAKind; };
                return HandType::ThreeOfAKind;
            },
            1 => {
                if count4 == 1 { return HandType::FiveOfAKind; };
                if count3 == 1 { return HandType::FourOfAKind; };
                if count2 == 2 { return HandType::FullHouse; };
                if count2 == 1 { return HandType::ThreeOfAKind; };
                return HandType::OnePair;
            }
            0 => {},
            _ => unreachable!()
        }

        if count5 == 1
        { return HandType::FiveOfAKind };

        if count4 == 1
        { return HandType::FourOfAKind };

        if count3 == 1 && count2 == 1
        { return HandType::FullHouse };

        if count3 == 1
        { return HandType::ThreeOfAKind };

        if count2 == 2
        { return HandType::TwoPair };

        if count2 == 1
        { return HandType::OnePair };

        if Self::count_n_of_a_kind::<1>(&cards) == 5
        { return HandType::HighCard };

        println!("Hand: {}", hand.to_string());
        unreachable!();
    }
}

#[test]
fn test_hand_type() {
    assert!(HandType::FullHouse < HandType::FourOfAKind);

    assert_eq!(HandType::count_n_of_a_kind::<5>(&Hand::from_str("AAAA2", Part1).cards), 0);
    assert_eq!(HandType::count_n_of_a_kind::<5>(&Hand::from_str("AAAAA", Part1).cards), 1);
    assert_eq!(HandType::count_n_of_a_kind::<4>(&Hand::from_str("AAAA2", Part1).cards), 1);
    assert_eq!(HandType::count_n_of_a_kind::<4>(&Hand::from_str("2AAAA", Part1).cards), 1);
    assert_eq!(HandType::count_n_of_a_kind::<4>(&Hand::from_str("AAAAA", Part1).cards), 0);
    assert_eq!(HandType::count_n_of_a_kind::<2>(&Hand::from_str("AA337", Part1).cards), 2);
    assert_eq!(HandType::count_n_of_a_kind::<2>(&Hand::from_str("7AA33", Part1).cards), 2);
    assert_eq!(HandType::count_n_of_a_kind::<1>(&Hand::from_str("23456", Part1).cards), 5);

    assert_eq!(HandType::count_joker(&Hand::from_str("AJAJ2", Part2).cards), 2);
    assert_eq!(HandType::count_joker(&Hand::from_str("JAJ4J", Part2).cards), 3);
    assert_eq!(HandType::count_n_of_a_kind::<5>(&Hand::from_str("JJJJJ", Part2).cards), 0);
    assert_eq!(HandType::count_n_of_a_kind::<5>(&Hand::from_str("AAAAA", Part2).cards), 1);
    assert_eq!(HandType::count_n_of_a_kind::<4>(&Hand::from_str("JJJJ2", Part2).cards), 0);
    assert_eq!(HandType::count_n_of_a_kind::<4>(&Hand::from_str("2AAAA", Part2).cards), 1);
    assert_eq!(HandType::count_n_of_a_kind::<2>(&Hand::from_str("AAJJ7", Part2).cards), 1);
    assert_eq!(HandType::count_n_of_a_kind::<1>(&Hand::from_str("2345J", Part2).cards), 4);

    assert_eq!(HandType::of(&Hand::from_str("AAAAA", Part1)), HandType::FiveOfAKind);
    assert_eq!(HandType::of(&Hand::from_str("AA8AA", Part1)), HandType::FourOfAKind);
    assert_eq!(HandType::of(&Hand::from_str("23332", Part1)), HandType::FullHouse);
    assert_eq!(HandType::of(&Hand::from_str("TTT98", Part1)), HandType::ThreeOfAKind);
    assert_eq!(HandType::of(&Hand::from_str("23432", Part1)), HandType::TwoPair);
    assert_eq!(HandType::of(&Hand::from_str("A23A4", Part1)), HandType::OnePair);
    assert_eq!(HandType::of(&Hand::from_str("23456", Part1)), HandType::HighCard);

    assert_eq!(HandType::of(&Hand::from_str("JJJJJ", Part2)), HandType::FiveOfAKind);
    assert_eq!(HandType::of(&Hand::from_str("JAJJJ", Part2)), HandType::FiveOfAKind);
    assert_eq!(HandType::of(&Hand::from_str("AAJJJ", Part2)), HandType::FiveOfAKind);
    assert_eq!(HandType::of(&Hand::from_str("A2JJJ", Part2)), HandType::FourOfAKind);
    assert_eq!(HandType::of(&Hand::from_str("JAAAJ", Part2)), HandType::FiveOfAKind);
    assert_eq!(HandType::of(&Hand::from_str("JAA3J", Part2)), HandType::FourOfAKind);
    assert_eq!(HandType::of(&Hand::from_str("JA43J", Part2)), HandType::ThreeOfAKind);
    assert_eq!(HandType::of(&Hand::from_str("AAAJA", Part2)), HandType::FiveOfAKind);
    assert_eq!(HandType::of(&Hand::from_str("A3AJA", Part2)), HandType::FourOfAKind);
    assert_eq!(HandType::of(&Hand::from_str("A33JA", Part2)), HandType::FullHouse);
    assert_eq!(HandType::of(&Hand::from_str("A34JA", Part2)), HandType::ThreeOfAKind);
    assert_eq!(HandType::of(&Hand::from_str("A34JT", Part2)), HandType::OnePair);

}

//////////////////////////////////////////
/// Hand Order
//////////////////////////////////////////

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        // different hand type?
        //println!("{} vs. {}: {:?}",self.to_string(), other.to_string(), HandType::of(self).cmp(&HandType::of(other)));
        match HandType::of(self).cmp(&HandType::of(other)) {
            Ordering::Less => { return Ordering::Less; }
            Ordering::Equal => {}
            Ordering::Greater => { return Ordering::Greater; }
        }
        for index in 0..5 {
            match self.cards[index].cmp(&other.cards[index]) {
                Ordering::Less => { return Ordering::Less; }
                Ordering::Equal => {}
                Ordering::Greater => { return Ordering::Greater; }
            }
        }
        //Ordering::Equal
        panic!("Two equal hands: {} == {}", self.to_string(), other.to_string())
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[test]
fn test_hand_order() {
    assert!(Hand::from_str("33332", Part1) > Hand::from_str("2AAAA", Part1));
    assert!(Hand::from_str("77888", Part1) > Hand::from_str("77788", Part1));
    assert!(Hand::from_str("AAAAK", Part1) > Hand::from_str("AAAAT", Part1));
    assert!(Hand::from_str("JJJJJ", Part1) > Hand::from_str("44444", Part1));
    assert!(Hand::from_str("JJJJJ", Part2) < Hand::from_str("44444", Part2));

    assert!(Hand::from_str("QQQJA", Part1) > Hand::from_str("T55J5", Part1));
    assert!(Hand::from_str("T55J5", Part1) > Hand::from_str("KK677", Part1));
    assert!(Hand::from_str("KK677", Part1) > Hand::from_str("KTJJT", Part1));
    assert!(Hand::from_str("KTJJT", Part1) > Hand::from_str("32T3K", Part1));

    assert!(Hand::from_str("KTJJT", Part2) > Hand::from_str("QQQJA", Part2));
    assert!(Hand::from_str("QQQJA", Part2) > Hand::from_str("T55J5", Part2));
    assert!(Hand::from_str("T55J5", Part2) > Hand::from_str("KK677", Part2));
    assert!(Hand::from_str("KK677", Part2) > Hand::from_str("32T3K", Part2));

}

#[test]
#[should_panic]
fn test_equal_cards() {
    // we don't expect two equal hands!
    let _ = Hand::from_str("T35KA", Part1) < Hand::from_str("T35KA", Part1);
}

//////////////////////////////////////////
/// Game
//////////////////////////////////////////

#[derive(Debug, PartialEq, Eq, Clone)]
struct HandWithBid {
    hand:Hand,
    bid:u32
}
type Game = Vec<HandWithBid>;

fn get_total_winning(game:&Game) -> usize {
    let mut sortedgame:Game = game.clone();
    sortedgame.sort_by(|game1, game2| game1.hand.cmp(&game2.hand));
    let mut sum = 0;
    for index in 0 .. sortedgame.len() {
        let rank = index + 1;
        let hand_with_bid = &sortedgame[index];
        let product = hand_with_bid.bid as usize * rank;
        //println!("{} * {} = {}", hand_with_bid.bid, rank, product);
        sum += product;
    }
    sum
}

#[cfg(test)]
fn example_game(part:Part) -> Game {
    vec![
        HandWithBid { hand: Hand::from_str("32T3K", part), bid: 765},
        HandWithBid { hand: Hand::from_str("T55J5", part), bid: 684},
        HandWithBid { hand: Hand::from_str("KK677", part), bid:  28},
        HandWithBid { hand: Hand::from_str("KTJJT", part), bid: 220},
        HandWithBid { hand: Hand::from_str("QQQJA", part), bid: 483}
    ]
}

#[test]
fn test_game() {
    let game = example_game(Part1);
    assert_eq!(get_total_winning(&game), 6440);
    let game = example_game(Part2);
    assert_eq!(get_total_winning(&game), 5905);
}

//////////////////////////////////////////
/// Input parsing
//////////////////////////////////////////

use pest::Parser;
use pest_derive::Parser;
use pest::iterators::Pair;

#[derive(Parser)]
#[grammar = "../grammar/day7.pest"]
struct Day7Parser;

fn build_game(file_rule:Pair<'_, Rule>, part:Part) -> Game {
    let mut game = Vec::new();
    let mut hand = Hand::from_str("55555", part);
    for column in file_rule.into_inner() {
        match column.as_rule() {
            Rule::cards => {
                hand = Hand::from_str(column.as_str(), part);
            },
            Rule::bid => {
                let bid = column.as_str().parse::<u32>().unwrap();
                game.push(HandWithBid{hand:hand.clone(), bid:bid});
            },
            Rule::EOI => {},
            _ => { println!("Unexpected {}", column); }
        }
    }
    game
}

#[test]
fn test_parse1() {
    assert_eq!(Day7Parser::parse(Rule::cards, "32T3K").unwrap().as_str(), "32T3K");
    assert_eq!(Day7Parser::parse(Rule::bid, "765").unwrap().as_str(), "765");

    let input =
"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

    assert_eq!(
        build_game(Day7Parser::parse(Rule::file, input).unwrap().next().unwrap(), Part1),
        example_game(Part1));

    assert_eq!(
        build_game(Day7Parser::parse(Rule::file, input).unwrap().next().unwrap(), Part2),
        example_game(Part2));

}

//////////////////////////////////////////
/// Productive usage
//////////////////////////////////////////

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn part1() {

    let file = File::open("data/day7.input").expect("Could not open data/day7.input");
    let reader = BufReader::new(file);

    let lines:Vec<String> = reader.lines().map( |line| line.unwrap() ).collect();
    let concat_input = lines.join("\n");

    for part in [Part1, Part2] {
        let mut parsed = Day7Parser::parse(Rule::file, &concat_input).unwrap();
        let file_rule = parsed.next().unwrap();
        let game = build_game(file_rule, part);

        println!("Day 7, {:?}: Total winnings of set of hands is {}", part, get_total_winning(&game));
    }

}