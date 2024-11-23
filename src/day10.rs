
#[derive(Clone, Copy, Debug)]
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