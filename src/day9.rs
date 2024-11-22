type Value = i32;

#[derive(Debug, PartialEq)]
struct History {
    history:Vec<Value>
}

impl History {
    fn from_str(input:&str) -> History {
        History { history:input.split(" ").map(|s| s.parse::<Value>().unwrap()).collect() }
    }

    fn last_known_value(&self) -> Value {
        self.history[self.history.len()-1]
    }

    fn predict(&self) -> Value {
        if self.all_zeroes() { return 0; }

        let diff = self.differentiate();
        return self.last_known_value() + diff.predict();
    }

    fn all_zeroes(&self) -> bool {
        !self.history.iter().any(|value| *value != 0)
    }

    // create new history [h1-h0, h2-h1, ...]
    fn differentiate(&self) -> History {
        let mut diff = Vec::<Value>::new();
        for i in 0..self.history.len()-1 {
            diff.push(self.history[i+1] - self.history[i]);
        }
        History { history:diff }
    }
}

#[test]
fn test_history() {
 
    let history1 = History::from_str("0 3 6 9 12 15");
    assert_eq!(history1, History { history:vec![0, 3, 6, 9, 12, 15]});
 
    assert_eq!(history1.all_zeroes(), false);
    let history1d = History::differentiate(&history1);
    assert_eq!(history1d, History::from_str("3 3 3 3 3"));
    assert!(!history1d.all_zeroes());
    let history1dd = History::differentiate(&history1d);
    assert_eq!(history1dd, History::from_str("0 0 0 0"));
    assert!(history1dd.all_zeroes());

    assert_eq!(history1.last_known_value(), 15);

    assert_eq!(history1dd.predict(), 0);
    assert_eq!(history1d.predict(), 3);
    assert_eq!(history1.predict(), 18);

    let history2 = History::from_str("1 3 6 10 15 21");
    let history2d = History::differentiate(&history2);
    assert_eq!(history2d, History::from_str("2 3 4 5 6"));
    assert_eq!(history2d.predict(), 7);
    assert_eq!(history2.predict(), 28);

    let history3 = History::from_str("10 13 16 21 30 45");
    let history3d = History::differentiate(&history3);
    assert_eq!(history3d, History::from_str("3 3 5 9 15"));
    assert_eq!(history3d.predict(), 23);
    assert_eq!(history3.predict(), 68);

}

//////////////////////////////////////////
/// Productive usage
//////////////////////////////////////////

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn part1() {

    let file = File::open("data/day9.input").expect("Could not open data/day9.input");
    let reader = BufReader::new(file);

    let lines:Vec<String> = reader.lines().map( |line| line.unwrap() ).collect();
    let sum_of_predictions:Value = lines.iter().map(|line| History::from_str(line).predict()).sum();
    println!("Day 9: Sum of predictions is {}", sum_of_predictions);

}