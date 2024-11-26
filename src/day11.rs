use std::fmt;
use std::collections::HashSet;

//////////////////////////////////////////
/// Galaxy
//////////////////////////////////////////


#[derive(PartialEq)]
struct Galaxy {
    x:u32,
    y:u32
}

impl fmt::Debug for Galaxy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

//////////////////////////////////////////
/// Space
//////////////////////////////////////////

struct Space {
    galaxies:Vec<Galaxy>,
    // expanding_lines count twice
    expanding_lines:HashSet<u32>,
    // expanding_rows count twice
    expanding_rows:HashSet<u32>
}

impl Space {
    fn pick_galaxies_from_image<'a>(lines:impl Iterator<Item=&'a str>) -> (Vec<Galaxy>,/*width: */u32,/*height: */u32) {
        let mut galaxies:Vec<Galaxy> = Vec::new();
        let mut y = 0;
        let mut x = 0;
        for line in lines {
            x = 0;
            for c in line.chars() {
                match c {
                    '.' => {},
                    '#' => { galaxies.push(Galaxy {x:x, y:y}); },
                    _ => panic!("Unexpected input character {}", c)
                }
                x += 1;
            }
            y += 1;
        }
        (galaxies, x, y)
    }

    fn from_image<'a>(lines:impl Iterator<Item=&'a str>) -> Space {
        let (galaxies, width, height) = Self::pick_galaxies_from_image(lines);
        let mut empty_lines:HashSet<u32> = HashSet::from_iter(0..height);
        let mut empty_rows :HashSet<u32> = HashSet::from_iter(0..width);
        for galaxy in &galaxies {
            empty_lines.remove(&galaxy.y);
            empty_rows.remove(&galaxy.x);
        }
        Space { galaxies:galaxies, expanding_lines:empty_lines, expanding_rows: empty_rows }
    }
}


#[test]
fn test_space() {
    let input1 =
"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
    let (galaxies1, w1, h1) = Space::pick_galaxies_from_image(input1.split("\n"));
    assert_eq!(h1, 10);
    assert_eq!(w1, 10);
    assert_eq!(galaxies1, vec![
        Galaxy {x: 3, y: 0},
        Galaxy {x: 7, y: 1},
        Galaxy {x: 0, y: 2},
        Galaxy {x: 6, y: 4},
        Galaxy {x: 1, y: 5},
        Galaxy {x: 9, y: 6},
        Galaxy {x: 7, y: 8},
        Galaxy {x: 0, y: 9},
        Galaxy {x: 4, y: 9},
    ]);

    let space1 = Space::from_image(input1.split("\n"));
    assert_eq!(space1.expanding_lines, HashSet::from([3, 7]));
    assert_eq!(space1.expanding_rows, HashSet::from([2, 5, 8]));
}
