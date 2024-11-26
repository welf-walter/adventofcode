use std::fmt;
use std::collections::HashSet;
use std::cmp::min;
use std::cmp::max;

//////////////////////////////////////////
/// Galaxy
//////////////////////////////////////////


#[derive(PartialEq, Clone, Copy)]
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

    fn size_of_line(&self, y:u32) -> u32 {
        if self.expanding_lines.contains(&y) { 2 } else { 1 }
    }

    fn size_of_row(&self, x:u32) -> u32 {
        if self.expanding_rows.contains(&x) { 2 } else { 1 }
    }

    fn distance(&self, galaxy1:Galaxy, galaxy2:Galaxy) -> u32 {
        (min(galaxy1.x, galaxy2.x) .. max(galaxy1.x, galaxy2.x)).map( |x| self.size_of_row(x)).sum::<u32>()
        +
        (min(galaxy1.y, galaxy2.y) .. max(galaxy1.y, galaxy2.y)).map( |y| self.size_of_line(y)).sum::<u32>()
    }

    fn distance_of_all_pairs(&self) -> u32 {
        let n = self.galaxies.len();
        let mut sum = 0;
        for i in 0 .. n {
            let galaxy1 = self.galaxies[i];
            for j in i + 1 .. n {
                let galaxy2 = self.galaxies[j];
                let distance = self.distance(galaxy1, galaxy2);
                println!("{} to {} is {}", i, j, distance);
                sum += distance;
            }
        }
        sum
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

    assert_eq!(space1.distance(space1.galaxies[5-1], space1.galaxies[9-1]), 9);
    assert_eq!(space1.distance(space1.galaxies[1-1], space1.galaxies[7-1]), 15);
    assert_eq!(space1.distance(space1.galaxies[3-1], space1.galaxies[6-1]), 17);
    assert_eq!(space1.distance(space1.galaxies[8-1], space1.galaxies[9-1]), 5);

    assert_eq!(space1.distance_of_all_pairs(), 374);
}
