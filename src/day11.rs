use std::fmt;

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

struct Space {
    galaxies:Vec<Galaxy>,
    // expanding_lines count twice
    expanding_lines:Vec<u32>,
    // expanding_rows count twice
    expanding_rows:Vec<u32>
}

impl Space {
    fn pick_galaxies_from_image<'a>(lines:impl Iterator<Item=&'a str>) -> Vec<Galaxy> {
        let mut galaxies:Vec<Galaxy> = Vec::new();
        let mut y = 0;
        for line in lines {
            let mut x = 0;
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
        galaxies
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
#...#.....
";
    let galaxies1 = Space::pick_galaxies_from_image(input1.split("\n"));
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

}
