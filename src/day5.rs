use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "../grammar/day5.pest"]
struct Day5Parser;

#[test]
fn test_parse() {
    let parse1 = Day5Parser::parse(Rule::number, "42").unwrap().peek().unwrap();
    assert_eq!(parse1.as_rule(), Rule::number);
    assert_eq!(parse1.as_str(), "42");

    assert_eq!(Day5Parser::parse(Rule::number, "6").unwrap().as_str(), "6");

    assert_eq!(Day5Parser::parse(Rule::seeds, "seeds: 4 15 76").unwrap().as_str(), "seeds: 4 15 76");

    assert_eq!(Day5Parser::parse(Rule::list_of_triples, "4 15 76\n1 2 3\n22 23 24").unwrap().as_str(), "4 15 76\n1 2 3\n22 23 24");

}

#[derive(PartialEq)]
#[derive(Debug)]
enum AlmanacType {
    Seed(u32),
    Soil(u32),
    Fertilizer(u32),
    Water(u32),
    Light(u32),
    Temperature(u32),
    Humidity(u32),
    Location(u32)
}

use AlmanacType::*;

trait AlmanacTypeTrait {
    fn to_u32(&self) -> u32;
    fn from_u32(&self, value:u32) -> Self;
}

impl AlmanacTypeTrait for AlmanacType {
    fn to_u32(&self) -> u32 {
        match self {
            Seed(value) => *value,
            Soil(value) => *value,
            Fertilizer(value) => *value,
            Water(value) => *value,
            Light(value) => *value,
            Temperature(value) => *value,
            Humidity(value) => *value,
            Location(value) => *value
        }
    }

    // not nice: I don't really need self, I would only need its type
    fn from_u32(&self, value: u32) -> Self {
        match self {
            Seed(_) => Seed(value),
            Soil(_) => Soil(value),
            Fertilizer(_) => Fertilizer(value),
            Water(_) => Water(value),
            Light(_) => Light(value),
            Temperature(_) => Temperature(value),
            Humidity(_) => Humidity(value),
            Location(_) => Location(value)
        }
    }
}

struct MappingRange<Destination:AlmanacTypeTrait, Source:AlmanacTypeTrait> {
//struct MappingRange<Destination, Source> {
    destination_range_start: Destination,
    source_range_start: Source,
    range_length: u32
}

impl<Destination:AlmanacTypeTrait, Source:AlmanacTypeTrait> MappingRange<Destination, Source> {
    fn is_source_in_range(&self, source:Source) -> bool {
        source.to_u32() >= self.source_range_start.to_u32()
        &&
        source.to_u32() < self.source_range_start.to_u32() + self.range_length
    }

    fn convert(&self, source:Source) -> Destination {
        self.destination_range_start. // <-- uuh. ugly! :(
        from_u32(
            self.destination_range_start.to_u32() +
            ( source.to_u32() - self.source_range_start.to_u32() ))
    }
}

#[test]
fn test_mapping_range() {
    //let range = MappingRange<Seed, Soil>{ map:vec![50, 98, 2]};
    let range = MappingRange{
        destination_range_start: Soil(50),
        source_range_start: Seed(98),
        range_length: 2};
    assert!(!range.is_source_in_range(Seed( 97)));
    assert!( range.is_source_in_range(Seed( 98)));
    assert!( range.is_source_in_range(Seed( 99)));
    assert!(!range.is_source_in_range(Seed(100)));

    assert_eq!(range.convert(Seed(98)), Soil(50));
    assert_eq!(range.convert(Seed(99)), Soil(51));
}
/*
struct Source_to_Destination_map<X:AlmanacType, Y:AlmanacType> {
    mapping_range_list:Vector<MappingRange>
}

fn convert<Source:AlmanacType, Destination:AlmanacType>(source:Source, map: &Source_to_Destination_map) {
    for range in map.mapping_range_list {
        if is_source_in_range()
    }
}
*/

#[test]
fn test_example1() {
    let input = [
        "seeds: 79 14 55 13",
        "",
        "seed-to-soil map:",
        "50 98 2",
        "52 50 48",
        "",
        "soil-to-fertilizer map:",
        "0 15 37",
        "37 52 2",
        "39 0 15",
        "",
        "fertilizer-to-water map:",
        "49 53 8",
        "0 11 42",
        "42 0 7",
        "57 7 4",
        "",
        "water-to-light map:",
        "88 18 7",
        "18 25 70",
        "",
        "light-to-temperature map:",
        "45 77 23",
        "81 45 19",
        "68 64 13",
        "",
        "temperature-to-humidity map:",
        "0 69 1",
        "1 0 69",
        "",
        "humidity-to-location map:",
        "60 56 37",
        "56 93 4"
    ];
    let parsed = Day5Parser::parse(Rule::file, &input.join("\n")).unwrap();
}

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn part1() {

    let file = File::open("data/day5.input").expect("Could not open data/day5.input");
    let reader = BufReader::new(file);

    let lines:Vec<String> = reader.lines().map( |line| line.unwrap() ).collect();
    let parsed = Day5Parser::parse(Rule::file, &lines.join("\n")).unwrap();

    println!("Parsing was successful");
}

