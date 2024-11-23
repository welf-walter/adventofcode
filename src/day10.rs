
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

#[derive(Debug, PartialEq)]
struct Position {
    x:u32,
    y:u32
}

const INVALID_POSITION:Position = Position { x:99999, y:99998};

//////////////////////////////////////////
/// Grid
//////////////////////////////////////////

struct Grid {
    tiles:Vec<Vec<Tile>>,
    start:Position,
    width:u32,
    height:u32
}

impl Grid {
    fn from_strings(lines:Vec<&str>) -> Grid {
        let mut tiles = Vec::new();
        let width = lines[0].len() as u32;
        let mut height = 0;
        let mut start = INVALID_POSITION;
        for line in lines {
            let tiles_line: Vec<Tile> = line.chars().map(|c| Tile::from_char(c)).collect();
            let search_start = tiles_line.iter().position(|tile| *tile == START_TILE);
            match search_start {
                Some(start_tile_pos) => start = Position{x:start_tile_pos as u32, y:height},
                None => {}
            }
            height += 1;
            assert_eq!(tiles_line.len() as u32, width);
            tiles.push(tiles_line);
        }
        Grid { tiles:tiles, start:start, width:width, height:height}
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
}