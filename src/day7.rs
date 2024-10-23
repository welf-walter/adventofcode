
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

#[derive(Debug, PartialEq, Eq)]
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
            cards[2] == cards[3] &&
            cards[3] != cards[4]) ||
           (cards[0] != cards[1] &&
            cards[1] == cards[2] &&
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
            cards[1] == cards[2] &&
            cards[2] != cards[3] &&
            cards[3] != cards[4]) ||
           (cards[0] != cards[1] &&
            cards[1] == cards[2] &&
            cards[2] == cards[3] &&
            cards[3] != cards[4]) ||
           (cards[0] != cards[1] &&
            cards[1] != cards[2] &&
            cards[2] == cards[3] &&
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
