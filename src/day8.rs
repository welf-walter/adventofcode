#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
struct Node(char, char, char);

impl Node {
    fn from_str(s:&str) -> Self {
        //s.chars().collect().try_into()
        if s.len() != 3 { panic!("Could not convert '{}' to node", s)};
        let mut iter = s.chars();
        Node(iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap())
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Right,
    Left
}

use Direction::*;

impl Direction {
    fn to_char(self) -> char {
        match self {
            Right => 'R',
            Left  => 'L'
        }
    }

    fn from_char(c:char) -> Self {
        match c {
            'R' => Right,
            'L' => Left,
            _ => unreachable!()
        }
    }

    fn from_str(s:&str) -> Vec<Self> {
        s.chars().map(|c| Self::from_char(c)).collect()
    }
}

#[test]
fn test_direction() {
    assert_eq!(Direction::from_char('L'), Left);
    assert_eq!(Direction::from_char('R'), Right);
    assert_eq!(Direction::to_char(Right), 'R');
    assert_eq!(Direction::to_char(Left), 'L');
    assert_eq!(Direction::from_str("LLR"), vec!(Left, Left, Right));
}

use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct Network {
    instructions:Vec<Direction>,
    map:HashMap<Node,(Node,Node)>
}

impl Network {
    fn insert_into_map(map:&mut HashMap<Node,(Node,Node)>, from: &str, left: &str, right: &str) {
        map.insert(Node::from_str(from), (Node::from_str(left), Node::from_str(right)));
    }

    fn walk(&self, from:Node, direction: Direction) -> Node {
        match direction {
            Left  => self.map.get(&from).unwrap().0,
            Right => self.map.get(&from).unwrap().1
        }
    }

    // how many steps does it take to walk from AAA to ZZZ?
    fn play(&self) -> u32 {
        let start  = Node::from_str("AAA");
        let finish = Node::from_str("ZZZ");

        let mut steps = 0;
        let mut node = start;
        loop {
            for direction in &self.instructions {
                node = self.walk(node, *direction);
                steps += 1;
                if node == finish {
                    return steps;
                }
            }
        }
    }
}

//////////////////////////////////////////
/// Input parsing
//////////////////////////////////////////

use pest::Parser;
use pest_derive::Parser;
use pest::iterators::Pair;

#[derive(Parser)]
#[grammar = "../grammar/day8.pest"]
struct Day8Parser;

#[test]
fn test_parse() {
    assert_eq!(Day8Parser::parse(Rule::instructions, "LLR").unwrap().as_str(), "LLR");
    assert_eq!(Day8Parser::parse(Rule::mapping, "AAA = (BBB, CCC)").unwrap().as_str(), "AAA = (BBB, CCC)");
}

fn build_network(file_rule:Pair<'_, Rule>) -> Network {
    let mut network:Network = Network{instructions:Vec::new(), map:HashMap::new()};

    for element in file_rule.into_inner() {
        match element.as_rule() {
            Rule::instructions => {
                network.instructions = Direction::from_str(element.as_str());
            },
            Rule::mapping => {
                let mut nodes = element.into_inner();
                while nodes.peek().expect("Could not peek").as_rule() == Rule::WHITESPACE { nodes.next(); }
                let from = nodes.next().unwrap();
                assert_eq!(from.as_rule(), Rule::node);

                while nodes.peek().expect("Could not peek").as_rule() == Rule::WHITESPACE { nodes.next(); }
                let left = nodes.next().unwrap();
                assert_eq!(left.as_rule(), Rule::node);

                while nodes.peek().expect("Could not peek").as_rule() == Rule::WHITESPACE { nodes.next(); }
                let right = nodes.next().unwrap();
                assert_eq!(right.as_rule(), Rule::node);

                Network::insert_into_map(&mut network.map, from.as_str(), left.as_str(), right.as_str());
            }
            Rule::EOI => {},
            _ => { println!("Unexpected {}", element); }
        }
    }
    network
}



//////////////////////////////////////////
/// Test Business Logic
//////////////////////////////////////////

#[cfg(test)]
fn example_network1() -> Network {
    Network {
        instructions:Direction::from_str("RL"),
        map:{
            let mut map = HashMap::new();
            Network::insert_into_map(&mut map, "AAA", "BBB", "CCC");
            Network::insert_into_map(&mut map, "BBB", "DDD", "EEE");
            Network::insert_into_map(&mut map, "CCC", "ZZZ", "GGG");
            Network::insert_into_map(&mut map, "DDD", "DDD", "DDD");
            Network::insert_into_map(&mut map, "EEE", "EEE", "EEE");
            Network::insert_into_map(&mut map, "GGG", "GGG", "GGG");
            Network::insert_into_map(&mut map, "ZZZ", "ZZZ", "ZZZ");
            map
        }
    }
}

#[cfg(test)]
fn example_network2() -> Network {
    Network {
        instructions:Direction::from_str("LLR"),
        map:{
            let mut map = HashMap::new();
            Network::insert_into_map(&mut map, "AAA", "BBB", "BBB");
            Network::insert_into_map(&mut map, "BBB", "AAA", "ZZZ");
            Network::insert_into_map(&mut map, "ZZZ", "ZZZ", "ZZZ");
            map
        }
    }
}

#[test]
fn test_network1() {
    let network = example_network1();
    assert_eq!(network.walk(Node::from_str("AAA"), Left ), Node::from_str("BBB"));
    assert_eq!(network.walk(Node::from_str("AAA"), Right), Node::from_str("CCC"));
    assert_eq!(network.walk(Node::from_str("BBB"), Left ), Node::from_str("DDD"));
    assert_eq!(network.walk(Node::from_str("BBB"), Right), Node::from_str("EEE"));

    assert_eq!(network.play(), 2);

    let input =
"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";
    println!("left = {:?}", build_network(Day8Parser::parse(Rule::file, input).unwrap().next().unwrap()));
    println!("right = {:?}", network);
    assert_eq!(
        build_network(Day8Parser::parse(Rule::file, input).unwrap().next().unwrap()),
        network);
}

#[test]
fn test_network2() {
    let network = example_network2();
    assert_eq!(network.walk(Node::from_str("AAA"), Left ), Node::from_str("BBB"));
    assert_eq!(network.walk(Node::from_str("AAA"), Right), Node::from_str("BBB"));
    assert_eq!(network.walk(Node::from_str("BBB"), Left ), Node::from_str("AAA"));
    assert_eq!(network.walk(Node::from_str("BBB"), Right), Node::from_str("ZZZ"));

    assert_eq!(network.play(), 6);

    let input =
"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";
    assert_eq!(
        build_network(Day8Parser::parse(Rule::file, input).unwrap().next().unwrap()),
        network);
}
