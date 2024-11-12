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
/// Node
//////////////////////////////////////////

#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
struct Node(char, char, char);

impl Node {
    fn from_str(s:&str) -> Self {
        //s.chars().collect().try_into()
        if s.len() != 3 { panic!("Could not convert '{}' to node", s)};
        let mut iter = s.chars();
        Node(iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap())
    }

    fn is_start_node(&self, part:Part) -> bool {
        match part {
            Part1 => self.0 == 'A' && self.1 == 'A' && self.2 == 'A',
            Part2 =>                                   self.2 == 'A'
        }
    }

    fn is_finish_node(&self, part:Part) -> bool {
        match part {
            Part1 => self.0 == 'Z' && self.1 == 'Z' && self.2 == 'Z',
            Part2 =>                                   self.2 == 'Z'
        }
    }

}

#[test]
fn test_node() {
    assert_eq!(Node::from_str("AAA").is_start_node(Part1), true);
    assert_eq!(Node::from_str("AAA").is_start_node(Part2), true);
    assert_eq!(Node::from_str("11A").is_start_node(Part1), false);
    assert_eq!(Node::from_str("11A").is_start_node(Part2), true);
    assert_eq!(Node::from_str("ZZZ").is_finish_node(Part1), true);
    assert_eq!(Node::from_str("ZZZ").is_finish_node(Part2), true);
    assert_eq!(Node::from_str("11Z").is_finish_node(Part1), false);
    assert_eq!(Node::from_str("11Z").is_finish_node(Part2), true);
}

//////////////////////////////////////////
/// Direction
//////////////////////////////////////////

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Right,
    Left
}

use Direction::*;

impl Direction {

    #[cfg(test)]
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

//////////////////////////////////////////
/// Network
//////////////////////////////////////////

use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct Network {
    instructions:Vec<Direction>,
    map:HashMap<Node,(Node,Node)>,
    start_nodes:Vec<Node>
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
    assert_eq!(Day8Parser::parse(Rule::mapping, "11A = (11B, XXX)").unwrap().as_str(), "11A = (11B, XXX)");
}

fn build_network(file_rule:Pair<'_, Rule>, part:Part) -> Network {
    let mut network:Network = Network{instructions:Vec::new(), map:HashMap::new(), start_nodes:Vec::new()};

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
                let from_node = Node::from_str(from.as_str());
                if from_node.is_start_node(part) { network.start_nodes.push(from_node);};
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
        },
        start_nodes:vec![Node::from_str("AAA")]
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
        },
        start_nodes:vec![Node::from_str("AAA")]
    }
}

#[cfg(test)]
fn example_network3() -> Network {
    let input =
    "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";
    build_network(Day8Parser::parse(Rule::file, input).unwrap().next().unwrap(), Part2)
}

#[test]
fn test_network1() {
    let network = example_network1();

    assert_eq!(network.start_nodes, vec![Node::from_str("AAA")]);

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
    let network_built = build_network(Day8Parser::parse(Rule::file, input).unwrap().next().unwrap(), Part1);
    println!("left = {:?}", network_built);
    println!("right = {:?}", network);
    assert_eq!(network_built, network);
}

#[test]
fn test_network2() {
    let network = example_network2();

    assert_eq!(network.start_nodes, vec![Node::from_str("AAA")]);

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
    let network_built = build_network(Day8Parser::parse(Rule::file, input).unwrap().next().unwrap(), Part1);
    println!("left = {:?}", network_built);
    println!("right = {:?}", network);
    assert_eq!(network_built, network);
}

#[test]
fn test_network3() {
    let network = example_network3();

    assert_eq!(network.start_nodes, vec![Node::from_str("11A"), Node::from_str("22A")]);
}

//////////////////////////////////////////
/// Productive usage
//////////////////////////////////////////

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn part1() {

    let file = File::open("data/day8.input").expect("Could not open data/day8.input");
    let reader = BufReader::new(file);

    let lines:Vec<String> = reader.lines().map( |line| line.unwrap() ).collect();
    let concat_input = lines.join("\n");
    // last \n is lost. I added one more newline at the end

    let mut parsed = Day8Parser::parse(Rule::file, &concat_input).unwrap();
    let file_rule = parsed.next().unwrap();
    let network = build_network(file_rule, Part1);

    let step_count = network.play();
    println!("Day 8: Number of steps is {}", step_count);

}