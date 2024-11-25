use std::fmt;

//////////////////////////////////////////
/// Tile
//////////////////////////////////////////

#[derive(Clone, Copy, Debug, PartialEq)]
struct Tile {
    c:char
/*

    | is a vertical pipe connecting north and south.
    - is a horizontal pipe connecting east and west.
    L is a 90-degree bend connecting north and east.
    J is a 90-degree bend connecting north and west.
    7 is a 90-degree bend connecting south and west.
    F is a 90-degree bend connecting south and east.
    . is ground; there is no pipe in this tile.
    S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.

*/
}

const START_TILE:Tile = Tile { c:'S' };

impl Tile {
    fn from_char(c:char) -> Tile {
        Tile { c:c }
    }

    fn connects_north(&self) -> bool {
        match self.c {
            '|' => true,
            '-' => false,
            'L' => true,
            'J' => true,
            '7' => false,
            'F' => false,
            '.' => false,
            'S' => panic!("I don't know"),
            _ => unreachable!()
        }
    }

    fn connects_east(&self) -> bool {
        match self.c {
            '|' => false,
            '-' => true,
            'L' => true,
            'J' => false,
            '7' => false,
            'F' => true,
            '.' => false,
            'S' => panic!("I don't know"),
            _ => unreachable!()
        }
    }

    fn connects_south(&self) -> bool {
        match self.c {
            '|' => true,
            '-' => false,
            'L' => false,
            'J' => false,
            '7' => true,
            'F' => true,
            '.' => false,
            'S' => panic!("I don't know"),
            _ => unreachable!()
        }
    }

    fn connects_west(&self) -> bool {
        match self.c {
            '|' => false,
            '-' => true,
            'L' => false,
            'J' => true,
            '7' => true,
            'F' => false,
            '.' => false,
            'S' => panic!("I don't know"),
            _ => unreachable!()
        }
    }

}

#[test]
fn test_tile() {
    assert!( Tile::from_char('|').connects_north());
    assert!(!Tile::from_char('|').connects_west());
    assert!( Tile::from_char('-').connects_east());
    assert!(!Tile::from_char('-').connects_south());
}

//////////////////////////////////////////
/// Position
//////////////////////////////////////////

#[derive(Debug, PartialEq, Copy, Clone)]
struct Position {
    x:usize,
    y:usize
}

const INVALID_POSITION:Position = Position { x:99999, y:99998};

impl Position {
    fn go(&self, direction:Direction) -> Position {
        match direction {
            NORTH => Position{x:self.x  ,y:self.y-1},
            EAST  => Position{x:self.x+1,y:self.y  },
            SOUTH => Position{x:self.x  ,y:self.y+1},
            WEST  => Position{x:self.x-1,y:self.y  }
        }
    }
}

//////////////////////////////////////////
/// Direction
//////////////////////////////////////////

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST
}

use Direction::*;

impl Direction {
    fn invert(&self) -> Self {
        match self {
            NORTH => SOUTH,
            EAST => WEST,
            SOUTH => NORTH,
            WEST => EAST
        }
    }
}

//////////////////////////////////////////
/// Grid
//////////////////////////////////////////

struct Grid {
    tiles:Vec<Vec<Tile>>,
    start:Position,
    width:usize,
    height:usize
}

impl Grid {
    fn from_strings(lines:Vec<&str>) -> Grid {
        let mut tiles = Vec::new();
        let width = lines[0].len();
        let mut height = 0;
        let mut start = INVALID_POSITION;
        for line in lines {
            let tiles_line: Vec<Tile> = line.chars().map(|c| Tile::from_char(c)).collect();
            let search_start = tiles_line.iter().position(|tile| *tile == START_TILE);
            match search_start {
                Some(start_tile_pos) => start = Position{x:start_tile_pos, y:height},
                None => {}
            }
            height += 1;
            assert_eq!(tiles_line.len(), width);
            tiles.push(tiles_line);
        }
        Grid { tiles:tiles, start:start, width:width, height:height}
    }

    fn get_tile(&self, position:Position) -> Tile {
        self.tiles[position.y][position.x]
    }

    fn walk(&self, position:Position, last_direction:Direction) -> Direction {
        let tile = self.get_tile(position);
        match last_direction {
            SOUTH /* coming from NORTH */ => {
                assert!(tile == START_TILE || tile.connects_north());
                if tile.connects_east()  {return EAST;}
                if tile.connects_south() {return SOUTH;}
                if tile.connects_west()  {return WEST;}
            },
            WEST /* coming from EAST */ => {
                assert!(tile == START_TILE || tile.connects_east());
                if tile.connects_north() {return NORTH;}
                if tile.connects_south() {return SOUTH;}
                if tile.connects_west()  {return WEST;}
            },
            NORTH /* coming from SOUTH */ => {
                assert!(tile == START_TILE || tile.connects_south());
                if tile.connects_north() {return NORTH;}
                if tile.connects_east()  {return EAST;}
                if tile.connects_west()  {return WEST;}
            },
            EAST /* coming from WEST */ => {
                assert!(tile == START_TILE || tile.connects_west());
                if tile.connects_north() {return NORTH;}
                if tile.connects_east()  {return EAST;}
                if tile.connects_south() {return SOUTH;}
            }
        }
        panic!("Cannot walk from ({}, {}) if I came from {:?}", position.x, position.y, last_direction);
    }
}

#[test]
fn test_grid() {
    let input1 =
".....
.S-7.
.|.|.
.L-J.
.....";
    let grid1 = Grid::from_strings(input1.split("\n").collect());
    assert_eq!(grid1.width, 5);
    assert_eq!(grid1.height, 5);
    assert_eq!(grid1.start, Position{x:1, y:1});
    assert_eq!(grid1.tiles,
        vec![
            vec![Tile::from_char('.'),Tile::from_char('.'),Tile::from_char('.'),Tile::from_char('.'),Tile::from_char('.')],
            vec![Tile::from_char('.'),Tile::from_char('S'),Tile::from_char('-'),Tile::from_char('7'),Tile::from_char('.')],
            vec![Tile::from_char('.'),Tile::from_char('|'),Tile::from_char('.'),Tile::from_char('|'),Tile::from_char('.')],
            vec![Tile::from_char('.'),Tile::from_char('L'),Tile::from_char('-'),Tile::from_char('J'),Tile::from_char('.')],
            vec![Tile::from_char('.'),Tile::from_char('.'),Tile::from_char('.'),Tile::from_char('.'),Tile::from_char('.')]
        ]);

    assert_eq!(Position{x:2, y:2}.go(NORTH), Position{x:2, y:1});
    assert_eq!(Position{x:2, y:2}.go(EAST),  Position{x:3, y:2});
    assert_eq!(Position{x:2, y:2}.go(SOUTH), Position{x:2, y:3});
    assert_eq!(Position{x:2, y:2}.go(WEST),  Position{x:1, y:2});

    assert_eq!(grid1.walk(Position{x:2, y:1}, EAST),  EAST);
    assert_eq!(grid1.walk(Position{x:3, y:1}, EAST),  SOUTH);
    assert_eq!(grid1.walk(Position{x:3, y:2}, SOUTH), SOUTH);
    assert_eq!(grid1.walk(Position{x:3, y:3}, SOUTH), WEST);
    assert_eq!(grid1.walk(Position{x:2, y:3}, WEST),  WEST);
    assert_eq!(grid1.walk(Position{x:1, y:3}, WEST),  NORTH);
    assert_eq!(grid1.walk(Position{x:1, y:2}, NORTH), NORTH);

}

//////////////////////////////////////////
/// Loop
//////////////////////////////////////////

#[derive(Debug, PartialEq)]
struct Step {
    position:Position,
    from:Direction,
    to:Direction
}

#[derive(Debug, PartialEq)]
struct Loop {
    steps:Vec<Step>
}

impl Loop {
    fn find_first_direction(grid:&Grid) -> (/* to: */Direction, /* from: */Direction) {
        assert_eq!(grid.get_tile(grid.start), START_TILE);

        let north = grid.get_tile(grid.start.go(NORTH)).connects_south();
        let east  = grid.get_tile(grid.start.go(EAST)).connects_west();
        let south = grid.get_tile(grid.start.go(SOUTH)).connects_north();
        let west  = grid.get_tile(grid.start.go(WEST)).connects_east();
        match (north, east, south, west) {
            (true,  true,  false, false) => (NORTH, EAST),
            (true,  false, true,  false) => (NORTH, SOUTH),
            (true,  false, false, true ) => (NORTH, WEST),
            (false, true,  true,  false) => (EAST,  SOUTH),
            (false, true,  false, true ) => (EAST,  WEST),
            (false, false, true,  true ) => (SOUTH, WEST),
            _ => { panic!("Unexpected combination north={:?}, east={:?}, south={:?}, west={:?}", north, east, south, west)}
        }
    }

    fn find_loop(grid:&Grid) -> Loop {
        let mut steps:Vec<Step> = Vec::new();

        let mut current = grid.start;
        let (mut next_direction, mut prev_direction) = Loop::find_first_direction(grid);

        loop {
            //println!("({}, {}): Go {:?}", current.x, current.y, next_direction);
            steps.push(Step{ position:current, to:next_direction, from:prev_direction});
            current = current.go(next_direction);
            if grid.get_tile(current) == START_TILE {
                return Loop {steps:steps};
            }
            prev_direction = next_direction.invert();
            next_direction = grid.walk(current, next_direction);
        }
    }

    fn get_distance_of_farthest_point(&self) -> usize {
        self.steps.len() / 2
    }
}

#[test]
fn test_loop() {
    let input1 =
".....
.S-7.
.|.|.
.L-J.
.....";
    let grid1 = Grid::from_strings(input1.split("\n").collect());
    let loop1 = Loop::find_loop(&grid1);
    assert_eq!(loop1.steps,vec![
        Step{position:Position{x:1, y:1}, from:SOUTH, to:EAST},
        Step{position:Position{x:2, y:1}, from:WEST,  to:EAST},
        Step{position:Position{x:3, y:1}, from:WEST,  to:SOUTH},
        Step{position:Position{x:3, y:2}, from:NORTH, to:SOUTH},
        Step{position:Position{x:3, y:3}, from:NORTH, to:WEST},
        Step{position:Position{x:2, y:3}, from:EAST,  to:WEST},
        Step{position:Position{x:1, y:3}, from:EAST,  to:NORTH},
        Step{position:Position{x:1, y:2}, from:SOUTH, to:NORTH}
    ]);
    assert_eq!(loop1.get_distance_of_farthest_point(), 4);

    let input2 =
"-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
    let grid2 = Grid::from_strings(input2.split("\n").collect());
    let loop2 = Loop::find_loop(&grid2);
    assert_eq!(loop1.steps, loop2.steps);
    assert_eq!(loop2.get_distance_of_farthest_point(), 4);

    let input3 =
"..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
    let grid3 = Grid::from_strings(input3.split("\n").collect());
    let loop3 = Loop::find_loop(&grid3);
    assert_eq!(loop3.get_distance_of_farthest_point(), 8);

    let input4 =
"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
    let grid4 = Grid::from_strings(input4.split("\n").collect());
    let loop4 = Loop::find_loop(&grid4);
    assert_eq!(loop3.steps, loop4.steps);
    assert_eq!(loop4.get_distance_of_farthest_point(), 8);

}

//////////////////////////////////////////
/// State
//////////////////////////////////////////

#[derive(Clone, Copy, Debug, PartialEq)]
enum State {
    Unknown,
    LoopVertical,
    LoopHorizontal,
    LoopEdgeNorth, // L or J
    LoopEdgeSouth, // F or 7
    Inside,
    Outside
}

impl State {
    fn to_char(&self) -> char {
      match self {
        State::Unknown => '.',
        State::LoopVertical => '|',
        State::LoopHorizontal => '-',
        State::LoopEdgeNorth => '^',
        State::LoopEdgeSouth => 'T',
        State::Inside => 'I',
        State::Outside => 'O'
      }
    }

    fn from_directions(from:Direction, to:Direction) -> State {
        match (from, to) {
            (NORTH, SOUTH) => State::LoopVertical,
            (SOUTH, NORTH) => State::LoopVertical,
            (WEST,  EAST ) => State::LoopHorizontal,
            (EAST,  WEST ) => State::LoopHorizontal,

            (NORTH, EAST ) => State::LoopEdgeNorth,
            (NORTH, WEST ) => State::LoopEdgeNorth,
            (EAST,  NORTH) => State::LoopEdgeNorth,
            (WEST,  NORTH) => State::LoopEdgeNorth,

            (SOUTH, EAST ) => State::LoopEdgeSouth,
            (SOUTH, WEST ) => State::LoopEdgeSouth,
            (EAST,  SOUTH) => State::LoopEdgeSouth,
            (WEST,  SOUTH) => State::LoopEdgeSouth,

            _ => panic!("Unexpected combination ({:?},{:?}", from, to)
        }
    }

}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

//////////////////////////////////////////
/// Enclosing
//////////////////////////////////////////

struct Enclosing {
    states:Vec<Vec<State>>
}

impl Enclosing {
    fn new(like_grid:&Grid) -> Enclosing {
        Enclosing { states: vec![vec![State::Unknown;like_grid.width]; like_grid.height] }
    }

    fn mark_loop(&mut self, the_loop:&Loop) {
        for step in &the_loop.steps {
            let state = State::from_directions(step.from, step.to);
            self.states[step.position.y][step.position.x] = state;
        }
    }

    fn mark_inside(&mut self) {
        let mut is_inside = false;
        let mut is_horizontal_pipe = false;
        let mut last_edge = State::Unknown;
        for line in self.states.iter_mut() {
            for state in line.iter_mut() {
                print!("{}{}{} ", state.to_char(), match is_inside { true => 'I', false => 'O' }, match is_horizontal_pipe { true => '-', false => ' ' });
                if is_horizontal_pipe {
                    match *state {
                        State::LoopHorizontal => { },
                        State::LoopEdgeNorth |
                        State::LoopEdgeSouth => {
                            is_horizontal_pipe = false;
                            if *state == last_edge {
                                // see "L--7" or "F--J" like "|"
                                is_inside = !is_inside;
                            }
                        },
                        _ => panic!("Unexpected state {} in horizontal line", *state)
                    }
                } else {
                    match *state {
                        State::Unknown => { *state = if is_inside { State::Inside } else { State::Outside }; },
                        State::LoopVertical => { is_inside = !is_inside },
                        State::LoopEdgeNorth |
                        State::LoopEdgeSouth => { is_horizontal_pipe = !is_horizontal_pipe; last_edge = *state },
                        _ => panic!("Unexpected state {}", *state)
                    }
                }
            }
            println!();
            assert!(!is_inside);
        }
    }
}

impl fmt::Display for Enclosing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in &self.states {
            for state in line {
                write!(f, "{}", state.to_char())?
            }
            writeln!(f, "")?
        }
        return Ok(());
    }
}

#[test]
fn test_enclosing() {
    let input1 =
"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
    let grid1 = Grid::from_strings(input1.split("\n").collect());
    let mut enclosing1 = Enclosing::new(&grid1);
    assert_eq!(enclosing1.to_string(),
"...........
...........
...........
...........
...........
...........
...........
...........
...........
");

    let loop1 = Loop::find_loop(&grid1);
    enclosing1.mark_loop(&loop1);
    assert_eq!(enclosing1.to_string(),
"...........
.+-------+.
.|+-----+|.
.||.....||.
.||.....||.
.|+-+.+-+|.
.|..|.|..|.
.+--+.+--+.
...........
");

    enclosing1.mark_inside();
    assert_eq!(enclosing1.to_string(),
"OOOOOOOOOOO
O+-------+O
O|+-----+|O
O||OOOOO||O
O||OOOOO||O
O|+-+O+-+|O
O|II|O|II|O
O+--+O+--+O
OOOOOOOOOOO
");
}


//////////////////////////////////////////
/// Productive usage
//////////////////////////////////////////

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn part1() {

    let file = File::open("data/day10.input").expect("Could not open data/day10.input");
    let reader = BufReader::new(file);

    let lines:Vec<String> = reader.lines().map( |line| line.unwrap() ).collect();
    let lines_ref:Vec<&str> = lines.iter().map( |line| line.as_str() ).collect();
    let grid = Grid::from_strings(lines_ref);
    assert_eq!(grid.width, 140);
    assert_eq!(grid.height, 140);
    let the_loop = Loop::find_loop(&grid);
    println!("Day 10, Part 1: Distance of farthest point {}", the_loop.get_distance_of_farthest_point());

}
