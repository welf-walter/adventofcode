type Value = i32;

#[derive(Debug, PartialEq)]
struct History {
    history:Vec<Value>
}

impl History {
    fn from_str(input:&str) -> History {
        History { history:input.split(" ").map(|s| s.parse::<Value>().unwrap()).collect() }
    }

    fn predict(&self) -> Value {
        -1
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

}
