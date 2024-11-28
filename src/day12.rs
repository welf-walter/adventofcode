use std::fmt;

//////////////////////////////////////////
/// Spring
//////////////////////////////////////////

#[derive(PartialEq, Clone, Copy, Debug)]
enum Spring {
    OPERATIONAL,
    DAMAGED,
    UNKNOWN
}

use Spring::*;

impl fmt::Display for Spring {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            OPERATIONAL => '.',
            DAMAGED     => '#',
            UNKNOWN     => '?'
        })
    }
}

impl Spring {
    fn from_char(c:char) -> Spring {
        match c {
            '.' => OPERATIONAL,
            '#' => DAMAGED,
            '?' => UNKNOWN,
            _   => unreachable!()
        }
    }
}


#[test]
fn test_display() {
    let spring = Spring::from_char('.');
    assert_eq!(spring, OPERATIONAL);
    assert_eq!(format!("{}", spring), ".");
}
