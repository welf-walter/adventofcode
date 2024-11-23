
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
            Direction::NORTH => Position{x:self.x  ,y:self.y-1},
            Direction::EAST  => Position{x:self.x+1,y:self.y  },
            Direction::SOUTH => Position{x:self.x  ,y:self.y+1},
            Direction::WEST  => Position{x:self.x-1,y:self.y  }
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
            Direction::SOUTH /* coming from NORTH */ => {
                assert!(tile.connects_north());
                if tile.connects_east()  {return Direction::EAST;}
                if tile.connects_south() {return Direction::SOUTH;}
                if tile.connects_west()  {return Direction::WEST;}
            },
            Direction::WEST /* coming from EAST */ => {
                assert!(tile.connects_east());
                if tile.connects_north() {return Direction::NORTH;}
                if tile.connects_south() {return Direction::SOUTH;}
                if tile.connects_west()  {return Direction::WEST;}
            },
            Direction::NORTH /* coming from SOUTH */ => {
                assert!(tile.connects_south());
                if tile.connects_north() {return Direction::NORTH;}
                if tile.connects_east()  {return Direction::EAST;}
                if tile.connects_west()  {return Direction::WEST;}
            },
            Direction::EAST /* coming from WEST */ => {
                assert!(tile.connects_west());
                if tile.connects_north() {return Direction::NORTH;}
                if tile.connects_east()  {return Direction::EAST;}
                if tile.connects_south() {return Direction::SOUTH;}
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

    assert_eq!(Position{x:2, y:2}.go(Direction::NORTH), Position{x:2, y:1});
    assert_eq!(Position{x:2, y:2}.go(Direction::EAST),  Position{x:3, y:2});
    assert_eq!(Position{x:2, y:2}.go(Direction::SOUTH), Position{x:2, y:3});
    assert_eq!(Position{x:2, y:2}.go(Direction::WEST),  Position{x:1, y:2});

    assert_eq!(grid1.walk(Position{x:2, y:1}, Direction::EAST),  Direction::EAST);
    assert_eq!(grid1.walk(Position{x:3, y:1}, Direction::EAST),  Direction::SOUTH);
    assert_eq!(grid1.walk(Position{x:3, y:2}, Direction::SOUTH), Direction::SOUTH);
    assert_eq!(grid1.walk(Position{x:3, y:3}, Direction::SOUTH), Direction::WEST);
    assert_eq!(grid1.walk(Position{x:2, y:3}, Direction::WEST),  Direction::WEST);
    assert_eq!(grid1.walk(Position{x:1, y:3}, Direction::WEST),  Direction::NORTH);
    assert_eq!(grid1.walk(Position{x:1, y:2}, Direction::NORTH), Direction::NORTH);

}
