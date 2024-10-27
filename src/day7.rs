
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
    _2
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
            Card::_2 => 2
        }
    }

    fn from_char(c:char) -> Card {
        match c {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
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

    #[cfg(test)]
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
            Card::_2 => '2'
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
    assert_eq!(Card::from_char('K'), Card::K);
    assert_eq!(Card::from_char('5'), Card::_5);
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
    fn from_str(s:&str) -> Hand {
        assert!(s.len()==5);
        let mut c5 = s.chars();
        Hand{cards:[Card::from_char(c5.next().unwrap()),
                    Card::from_char(c5.next().unwrap()),
                    Card::from_char(c5.next().unwrap()),
                    Card::from_char(c5.next().unwrap()),
                    Card::from_char(c5.next().unwrap())]}
    }

    #[cfg(test)]
    fn to_char5(&self) -> [char;5] {
        [
            self.cards[0].to_char(),
            self.cards[1].to_char(),
            self.cards[2].to_char(),
            self.cards[3].to_char(),
            self.cards[4].to_char()
        ]
    }

    #[cfg(test)]
    fn to_string(&self) -> String {
        self.to_char5().iter().collect()
    }
}

#[test]
fn test_hand() {
    assert_eq!(Hand::from_str("32T3K"), Hand{cards:[Card::_3, Card::_2, Card::T, Card::_3, Card::K]});
    assert_eq!(Hand::from_str("32T3K").to_char5(), ['3', '2', 'T', '3', 'K']);
    assert_eq!(Hand::from_str("32T3K").to_string(), "32T3K");
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
    fn of(hand:&Hand) -> Self {
        let mut cards = hand.cards.clone();
        cards.sort();

        if cards[0] == cards[1] &&
           cards[1] == cards[2] &&
           cards[2] == cards[3] &&
           cards[3] == cards[4]
        { return HandType::FiveOfAKind };

        if (cards[0] == cards[1] &&
            cards[1] == cards[2] &&
            cards[2] == cards[3]) ||
           (cards[1] == cards[2] &&
            cards[2] == cards[3] &&
            cards[3] == cards[4])
        { return HandType::FourOfAKind };

        if (cards[0] == cards[1] &&
            cards[1] == cards[2] &&
            cards[2] != cards[3] &&
            cards[3] == cards[4]) ||
           (cards[0] == cards[1] &&
            cards[1] != cards[2] &&
            cards[2] == cards[3] &&
            cards[3] == cards[4])
        { return HandType::FullHouse };

        if (cards[0] == cards[1] &&
            cards[1] == cards[2]) ||
           (cards[1] == cards[2] &&
            cards[2] == cards[3]) ||
           (cards[2] == cards[3] &&
            cards[3] == cards[4])
        { return HandType::ThreeOfAKind };

        if (cards[0] == cards[1] && cards[2] == cards[3])
        || (cards[0] == cards[1] && cards[3] == cards[4])
        || (cards[1] == cards[2] && cards[3] == cards[4])
        { return HandType::TwoPair };

        if (cards[0] == cards[1])
        || (cards[1] == cards[2])
        || (cards[2] == cards[3])
        || (cards[3] == cards[4])
        { return HandType::OnePair };

        if cards[0] != cards[1] &&
           cards[1] != cards[2] &&
           cards[2] != cards[3] &&
           cards[3] != cards[4]
        { return HandType::HighCard };

        unreachable!();
    }
}

#[test]
fn test_hand_type() {
    assert!(HandType::FullHouse < HandType::FourOfAKind);
    assert_eq!(HandType::of(&Hand::from_str("AAAAA")), HandType::FiveOfAKind);
    assert_eq!(HandType::of(&Hand::from_str("AA8AA")), HandType::FourOfAKind);
    assert_eq!(HandType::of(&Hand::from_str("23332")), HandType::FullHouse);
    assert_eq!(HandType::of(&Hand::from_str("TTT98")), HandType::ThreeOfAKind);
    assert_eq!(HandType::of(&Hand::from_str("23432")), HandType::TwoPair);
    assert_eq!(HandType::of(&Hand::from_str("A23A4")), HandType::OnePair);
    assert_eq!(HandType::of(&Hand::from_str("23456")), HandType::HighCard);
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
        for index in 0..4 {
            match self.cards[index].cmp(&other.cards[index]) {
                Ordering::Less => { return Ordering::Less; }
                Ordering::Equal => {}
                Ordering::Greater => { return Ordering::Greater; }
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[test]
fn test_hand_order() {
    assert!(Hand::from_str("33332") > Hand::from_str("2AAAA"));
    assert!(Hand::from_str("77888") > Hand::from_str("77788"));

    assert!(Hand::from_str("QQQJA") > Hand::from_str("T55J5"));
    assert!(Hand::from_str("T55J5") > Hand::from_str("KK677"));
    assert!(Hand::from_str("KK677") > Hand::from_str("KTJJT"));
    assert!(Hand::from_str("KTJJT") > Hand::from_str("32T3K"));
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
fn example_game() -> Game {
    vec![
        HandWithBid { hand: Hand::from_str("32T3K"), bid: 765},
        HandWithBid { hand: Hand::from_str("T55J5"), bid: 684},
        HandWithBid { hand: Hand::from_str("KK677"), bid:  28},
        HandWithBid { hand: Hand::from_str("KTJJT"), bid: 220},
        HandWithBid { hand: Hand::from_str("QQQJA"), bid: 483}
    ]
}

#[test]
fn test_game() {
    let game = example_game();
    assert_eq!(get_total_winning(&game), 6440);
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

fn build_game(file_rule:Pair<'_, Rule>) -> Game {
    let mut game = Vec::new();
    let mut hand = Hand::from_str("55555");
    for column in file_rule.into_inner() {
        match column.as_rule() {
            Rule::cards => {
                hand = Hand::from_str(column.as_str());
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
        build_game(Day7Parser::parse(Rule::file, input).unwrap().next().unwrap()),
        example_game());

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
    let mut parsed = Day7Parser::parse(Rule::file, &concat_input).unwrap();
    let file_rule = parsed.next().unwrap();
    let game = build_game(file_rule);

    println!("Day 7, part 1: Total winnings of set of hands is {}", get_total_winning(&game));

}