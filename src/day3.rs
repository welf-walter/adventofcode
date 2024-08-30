use regex::Regex;
use std::cmp;
use std::collections::HashMap;

struct EngineSchematic {
    lines:Vec<String>
}

#[derive(PartialEq)]
#[derive(Debug)]
struct Number {
    line:usize,
    // first column of the number
    from:usize,
    // last column of the number
    to:usize,
    value:u32
}

fn extract_numbers(engine_schematic:&EngineSchematic) -> Vec<Number> {
    let mut numbers=Vec::new();
    let re = Regex::new(r"(\d+)").unwrap();
    let mut line_number = 0;
    for line in &engine_schematic.lines {
        for number_match in re.find_iter(&line) {
            let value = number_match.as_str().parse::<u32>().unwrap();
            numbers.push(
                Number { 
                    line:  line_number,
                    from:  number_match.start(),
                    to:    number_match.end()-1,
                    value: value
                }
            );
        }
        line_number += 1;
    }
    numbers
}

#[test]
fn test_regex() {
    let re_any_symbol = Regex::new(r"[^0123456789\.]").unwrap();
    assert!(!re_any_symbol.is_match("......"));    
    assert!(!re_any_symbol.is_match("..343.."));    
    assert!(re_any_symbol.is_match("...*.."));
    assert!(re_any_symbol.is_match("...+.."));    
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Symbol {
    line:usize,
    column:usize,
}

fn find_symbol(engine_schematic:&EngineSchematic, fromline:isize, toline:usize, fromcol: isize, tocol: usize) -> Option<Symbol> {
    let fromline_safe = cmp::max(fromline, 0) as usize;
    let fromcol_safe  = cmp::max(fromcol, 0) as usize;
    let toline_safe   = cmp::min(toline, engine_schematic.lines.len() - 1);
    let tocol_safe    = cmp::min(tocol, engine_schematic.lines[0].len() - 1);
    let re_any_symbol = Regex::new(r"[^0123456789\.]").unwrap();
    //println!("Line from {} to {}", fromline_safe, toline_safe);

    for lineindex in fromline_safe..toline_safe+1 {
        let line = &engine_schematic.lines[lineindex];
        let section = &line[fromcol_safe..tocol_safe+1];
        //println!("  Search from {} to {} in {}: {}", fromcol_safe, tocol_safe, line, section);
        if let Some(symbolmatch) = re_any_symbol.find(section) {
            //println!("  Found!");
            let column = fromcol_safe+symbolmatch.start();
            return Some(Symbol{line:lineindex, column:column});
        }
    }
    None
}

fn is_part_number(engine_schematic:&EngineSchematic, number:&Number) -> bool {
    match find_symbol(engine_schematic, number.line as isize - 1, number.line + 1, number.from as isize - 1, number.to + 1) {
        Some(_symbol) => true,
        None => false
    }
}

#[test]
fn examples1() {
    let engi = EngineSchematic{
        lines: [
            String::from("467..114.."),
            String::from("...*......"),
            String::from("..35..633."),
            String::from("......#..."),
            String::from("617*......"),
            String::from(".....+.58."),
            String::from("..592....."),
            String::from("......755."),
            String::from("...$.*...."),
            String::from(".664.598..")
        ].to_vec()
    };
    assert_eq!(engi.lines[3], "......#...");

    let numbers = extract_numbers(&engi);
    assert_eq!(numbers.len(), 10);
    assert_eq!(numbers[2], Number{line:2, from: 2, to: 3, value: 35});
    assert_eq!(numbers[3], Number{line:2, from: 6, to: 8, value: 633});

    assert_eq!(find_symbol(&engi, -1, 1, -1, 3), Some(Symbol{line:1, column:3}));
    assert_eq!(find_symbol(&engi, -1, 0, -1, 3), None);
    assert_eq!(find_symbol(&engi, -1, 1, -1, 2), None);
    assert_eq!(find_symbol(&engi, 2, 4, 6, 8), Some(Symbol{line:3, column:6}));
    assert_eq!(find_symbol(&engi, 2, 4, 7, 8), None);
    assert_eq!(find_symbol(&engi, 2, 4, 4, 6), Some(Symbol{line:3, column:6}));
    assert_eq!(find_symbol(&engi, 2, 4, 4, 5), None);

    assert_eq!(is_part_number(&engi, &numbers[0]), true );
    assert_eq!(is_part_number(&engi, &numbers[1]), false );
    assert_eq!(is_part_number(&engi, &numbers[2]), true );
    assert_eq!(is_part_number(&engi, &numbers[3]), true );
    assert_eq!(is_part_number(&engi, &numbers[4]), true );
    assert_eq!(is_part_number(&engi, &numbers[5]), false );
    assert_eq!(is_part_number(&engi, &numbers[6]), true );
    assert_eq!(is_part_number(&engi, &numbers[7]), true );
    assert_eq!(is_part_number(&engi, &numbers[8]), true );
    assert_eq!(is_part_number(&engi, &numbers[9]), true );

}

struct Adjacencies {
    // store for one symbol the list of all adjacent numbers
    map: HashMap<Symbol, Vec<Number>>
}

fn get_adjacencies(engi: &EngineSchematic) -> Adjacencies {
    let mut adj:Adjacencies = Adjacencies{map:HashMap::new()};
    let numbers = extract_numbers(&engi);

    for number in numbers {
        let adjacent_symbol = find_symbol(engi, number.line as isize - 1, number.line + 1, number.from as isize - 1, number.to + 1);
        if let Some(symbol) = adjacent_symbol {
            adj.map.entry(symbol).or_insert(Vec::new()).push(number);
        }
    }
    adj
}

#[test]
fn examples2() {
    let engi = EngineSchematic{
        lines: [
            String::from("467..114.."),
            String::from("...*......"),
            String::from("..35..633."),
            String::from("......#..."),
            String::from("617*......"),
            String::from(".....+.58."),
            String::from("..592....."),
            String::from("......755."),
            String::from("...$.*...."),
            String::from(".664.598..")
        ].to_vec()
    };
    let adj = get_adjacencies(&engi);
    assert_eq!(adj.map.len(), 6);
    let sym1nums = adj.map.get(&Symbol{line:1, column: 3}).unwrap();
    assert_eq!(sym1nums.len(), 2);
    assert_eq!(sym1nums.get(0).unwrap().value, 467);
    assert_eq!(sym1nums.get(1).unwrap().value, 35);

    let sym2nums = adj.map.get(&Symbol{line:8, column: 5}).unwrap();
    assert_eq!(sym2nums.len(), 2);
    assert_eq!(sym2nums.get(0).unwrap().value, 755);
    assert_eq!(sym2nums.get(1).unwrap().value, 598);

    let sym3nums = adj.map.get(&Symbol{line:4, column: 3}).unwrap();
    assert_eq!(sym3nums.len(), 1);
    assert_eq!(sym3nums.get(0).unwrap().value, 617);

}


// -----------------------------------------------------------------------------------

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn part1() {
    let file = File::open("data/day3.input").expect("Could not open data/day3.input");
    let reader = BufReader::new(file);

    let engi = EngineSchematic{ lines: reader.lines().map(|line| line.expect("line failure")).collect() };
    let numbers = extract_numbers(&engi);
    let mut sum_of_part_numbers = 0;
    let mut cnt_of_part_numbers = 0;
    let mut cnt_of_numbers = 0;

    for number in numbers {
        if is_part_number(&engi, &number) {
            sum_of_part_numbers += number.value;
            cnt_of_part_numbers += 1;
        }
        cnt_of_numbers += 1;
    }

    println!("Day 3: {} of {} numbers are parts. Their sum = {}.",
      cnt_of_part_numbers, cnt_of_numbers,
      sum_of_part_numbers);
}
