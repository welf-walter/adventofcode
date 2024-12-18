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

type Steps = u64;

//////////////////////////////////////////
/// Node
//////////////////////////////////////////

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Node(char, char, char);

use std::fmt;
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}", self.0, self.1, self.2)
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}{}{}", self.0, self.1, self.2)
    }
}

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
    assert_eq!(format!("{}", Node::from_str("ABC")), "ABC");
    assert_eq!(format!("{:?}", Node::from_str("ABC")), "ABC");
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

    fn instructions_len(&self) -> Steps {
        self.instructions.len() as Steps
    }

    // follow the routes from one nodes until the end
    fn follow_routes(&self, routes:&HashMap<Node,Route>, from:Node, part:Part) -> (/*steps: */Steps, /*target: */Node) {
        let mut current_node = from;
        let mut step_count = 0;
        loop {
            let route = routes.get(&current_node).unwrap();
            current_node = route.target_node;
            step_count += self.instructions_len();

            if current_node.is_finish_node(part) {
                return (step_count, current_node );
            }
        }
    }

    // while not strictly in the rules of the game, it seems that the actual network of the
    // puzzle is quite nice:
    // From a start node to the target node the FULL instruction set is executed several times (so e.g. no target after 2 full runs and 3 single instructions)
    // If the start node reaches target node ZZZ after n steps, the target node ZZZ is reached AGAIN after n steps
    fn check_network_is_nice(&self, part:Part) {
        let routes = Route::generate_all_routes(self, part);

        for start_node in &self.start_nodes {

            let (steps1, target1) = self.follow_routes(&routes, *start_node, part);
            //println!("  From {} to {} in {} steps.", start_node, target1, steps1);

            let (steps2, target2) = self.follow_routes(&routes, target1, part);
            //println!("    From {} to {} in {} steps.", target1, target2, steps2);

            assert_eq!(steps1, steps2);
            assert_eq!(target1, target2);

        }
    }

    // how many steps does it take to walk from AAA to ZZZ?
    fn play(&self, part:Part) -> Steps {

        let routes = Route::generate_all_routes(self, part);
        let mut steps_per_startnode = Vec::new();

        for start_node in &self.start_nodes {

            let (steps, _target) = self.follow_routes(&routes, *start_node, part);
            //println!("  From {} to {} in {} steps.", start_node, target, steps);
            steps_per_startnode.push(steps);

        }

        let mut lowest_common_multiple = 1;
        for steps in steps_per_startnode {
            lowest_common_multiple =  num::integer::lcm(steps, lowest_common_multiple);
        }

        lowest_common_multiple

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
/// Route
//////////////////////////////////////////

#[derive(Debug, PartialEq)]
// Walk once all instructions through the network
struct Route<'a> {
    network:&'a Network,
    // start at this node
    start_node:Node,
    // end at this node
    target_node:Node,
    // reach a finish node after so many steps
    target_is_finish:bool
}

impl fmt::Display for Route<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {} (is_finish = {})", self.start_node, self.target_node, self.target_is_finish)
    }
}

impl Route<'_> {

    fn generate_route(network:&Network, start_node:Node, part:Part) -> Route {
        let mut current_node = start_node;
        //let mut step_count = 0;
        for direction in &network.instructions {
            //step_count += 1;
            current_node = network.walk(current_node, *direction);

            // in fact, the puzzle is simpler than expected:
            // the finish is only reachable at the end of one instruction set
            // make sure this is true
            //if current_node.is_finish_node(part) {
            //    if step_count != network.instructions_len() {
            //        panic!("I thought only at the end of an instruction set a finish node is reached: node = {}, step = {}", current_node, step_count);
            //    }
            //}
            // in fact the assumption above fails for the test cases!
        }
        Route {
            network:network,
            start_node:start_node,
            target_node:current_node,
            target_is_finish:current_node.is_finish_node(part)
        }
    }

    fn generate_all_routes(network:&Network, part:Part) -> HashMap<Node,Route> {
        let mut nodes_to_process = network.start_nodes.clone();
        let mut routes:HashMap<Node,Route> = HashMap::new();
        while nodes_to_process.len() > 0 {
            let node = nodes_to_process.pop().expect("nodes_to_process is empty");
            let route = Self::generate_route(network, node, part);
            //println!("{}", &route);
            if !routes.contains_key(&route.target_node) {
                nodes_to_process.push(route.target_node);
            }
            routes.insert(node, route);
        }
        routes
    }

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
    network.check_network_is_nice(Part1);

    assert_eq!(network.start_nodes, vec![Node::from_str("AAA")]);

    assert_eq!(network.walk(Node::from_str("AAA"), Left ), Node::from_str("BBB"));
    assert_eq!(network.walk(Node::from_str("AAA"), Right), Node::from_str("CCC"));
    assert_eq!(network.walk(Node::from_str("BBB"), Left ), Node::from_str("DDD"));
    assert_eq!(network.walk(Node::from_str("BBB"), Right), Node::from_str("EEE"));

    assert_eq!(network.play(Part1), 2);

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

    let route1 = Route::generate_route(&network, Node::from_str("AAA"), Part1);
    assert_eq!(route1.target_node, Node::from_str("ZZZ"));
    assert_eq!(route1.target_is_finish, true);

    let route2 = Route::generate_route(&network, Node::from_str("ZZZ"), Part1);
    assert_eq!(route2.target_node, Node::from_str("ZZZ"));
    assert_eq!(route2.target_is_finish, true);

}

#[test]
fn test_network2() {
    let network = example_network2();

    // actually, the example network 2 does not adhere to the assumed
    // simplifications necessary to calculate the result in that way :(
    //network.check_network_is_nice(Part1);

    assert_eq!(network.start_nodes, vec![Node::from_str("AAA")]);

    assert_eq!(network.walk(Node::from_str("AAA"), Left ), Node::from_str("BBB"));
    assert_eq!(network.walk(Node::from_str("AAA"), Right), Node::from_str("BBB"));
    assert_eq!(network.walk(Node::from_str("BBB"), Left ), Node::from_str("AAA"));
    assert_eq!(network.walk(Node::from_str("BBB"), Right), Node::from_str("ZZZ"));

    assert_eq!(network.play(Part1), 6);

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

    let route1 = Route::generate_route(&network, Node::from_str("AAA"), Part1);
    assert_eq!(route1.target_node, Node::from_str("BBB"));
    assert_eq!(route1.target_is_finish, false);

    let route2 = Route::generate_route(&network, Node::from_str("BBB"), Part1);
    assert_eq!(route2.target_node, Node::from_str("ZZZ"));
    assert_eq!(route2.target_is_finish, true);

    let routes = Route::generate_all_routes(&network, Part1);
    assert_eq!(routes.len(), 3);
    assert_eq!(routes.get(&Node::from_str("AAA")).unwrap().target_node, Node::from_str("BBB"));
    assert_eq!(routes.get(&Node::from_str("BBB")).unwrap().target_node, Node::from_str("ZZZ"));
    assert_eq!(routes.get(&Node::from_str("ZZZ")).unwrap().target_node, Node::from_str("ZZZ"));

}

#[test]
fn test_network3() {
    let network = example_network3();
    network.check_network_is_nice(Part2);

    assert_eq!(network.start_nodes, vec![Node::from_str("11A"), Node::from_str("22A")]);

    assert_eq!(network.walk(Node::from_str("11A"), Left ), Node::from_str("11B"));
    assert_eq!(network.walk(Node::from_str("11A"), Right), Node::from_str("XXX"));
    assert_eq!(network.walk(Node::from_str("22A"), Left ), Node::from_str("22B"));
    assert_eq!(network.walk(Node::from_str("22A"), Right), Node::from_str("XXX"));

    assert_eq!(network.play(Part2), 6);

    let routes = Route::generate_all_routes(&network, Part2);
    assert_eq!(routes.len(), 6);
    assert_eq!(routes.get(&Node::from_str("11A")).unwrap().target_node, Node::from_str("11Z"));
    assert_eq!(routes.get(&Node::from_str("11Z")).unwrap().target_node, Node::from_str("11Z"));
    assert_eq!(routes.get(&Node::from_str("22A")).unwrap().target_node, Node::from_str("22C"));
    assert_eq!(routes.get(&Node::from_str("22C")).unwrap().target_node, Node::from_str("22B"));
    assert_eq!(routes.get(&Node::from_str("22B")).unwrap().target_node, Node::from_str("22Z"));
    assert_eq!(routes.get(&Node::from_str("22Z")).unwrap().target_node, Node::from_str("22C"));

}

//////////////////////////////////////////
/// Productive usage
//////////////////////////////////////////

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn part1and2() {

    let file = File::open("data/day8.input").expect("Could not open data/day8.input");
    let reader = BufReader::new(file);

    let lines:Vec<String> = reader.lines().map( |line| line.unwrap() ).collect();
    let concat_input = lines.join("\n");
    // last \n is lost. I added one more newline at the end

    for part in [Part1, Part2] {
        let mut parsed = Day8Parser::parse(Rule::file, &concat_input).unwrap();
        let file_rule = parsed.next().unwrap();
        let network = build_network(file_rule, part);
        network.check_network_is_nice(part);

        //let routes = Route::generate_all_routes(&network, part);
        //for (_, route) in routes.iter() {
        //    println!("       {}", route);
        //}

        let step_count = network.play(part);
        println!("Day 8, {:?}: Number of steps is {}", part, step_count);
    }

}