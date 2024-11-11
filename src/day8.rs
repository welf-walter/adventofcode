#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
struct Node(char, char, char);

impl Node {
    fn from_str(s:&str) -> Self {
        //s.chars().collect().try_into()
        assert_eq!(s.len(), 3);
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
}

#[test]
fn test_network2() {
    let network = example_network2();
    assert_eq!(network.walk(Node::from_str("AAA"), Left ), Node::from_str("BBB"));
    assert_eq!(network.walk(Node::from_str("AAA"), Right), Node::from_str("BBB"));
    assert_eq!(network.walk(Node::from_str("BBB"), Left ), Node::from_str("AAA"));
    assert_eq!(network.walk(Node::from_str("BBB"), Right), Node::from_str("ZZZ"));

    assert_eq!(network.play(), 6);
}
