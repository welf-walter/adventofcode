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
#[derive(Clone)]
#[derive(Copy)]
struct Seed(u32);

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
struct Soil(u32);

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
struct Fertilizer(u32);

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
struct Water(u32);

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
struct Light(u32);

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
struct Temperature(u32);

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
struct Humidity(u32);

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
struct Location(u32);

trait AlmanacTypeTrait {
    fn to_u32(&self) -> u32;
    fn from_u32(value:u32) -> Self;
}

impl AlmanacTypeTrait for Seed {
    fn to_u32(&self) -> u32 { self.0 }
    fn from_u32(value:u32) -> Self { Self(value )}
}

impl AlmanacTypeTrait for Soil {
    fn to_u32(&self) -> u32 { self.0 }
    fn from_u32(value:u32) -> Self { Self(value )}
}

impl AlmanacTypeTrait for Fertilizer {
    fn to_u32(&self) -> u32 { self.0 }
    fn from_u32(value:u32) -> Self { Self(value )}
}

impl AlmanacTypeTrait for Water {
    fn to_u32(&self) -> u32 { self.0 }
    fn from_u32(value:u32) -> Self { Self(value )}
}

impl AlmanacTypeTrait for Light {
    fn to_u32(&self) -> u32 { self.0 }
    fn from_u32(value:u32) -> Self { Self(value )}
}

impl AlmanacTypeTrait for Temperature {
    fn to_u32(&self) -> u32 { self.0 }
    fn from_u32(value:u32) -> Self { Self(value )}
}

impl AlmanacTypeTrait for Humidity {
    fn to_u32(&self) -> u32 { self.0 }
    fn from_u32(value:u32) -> Self { Self(value )}
}

impl AlmanacTypeTrait for Location {
    fn to_u32(&self) -> u32 { self.0 }
    fn from_u32(value:u32) -> Self { Self(value )}
}

struct MappingRange<Destination:AlmanacTypeTrait, Source:AlmanacTypeTrait> {
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
        Destination::from_u32(
            self.destination_range_start.to_u32() +
            ( source.to_u32() - self.source_range_start.to_u32() ))
    }
}

#[test]
fn test_mapping_range() {
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
struct SourceToDestinationMap<Source:AlmanacTypeTrait, Destination:AlmanacTypeTrait> {
    mapping_range_list:Vec<MappingRange<Destination, Source>>
}

impl<Source:AlmanacTypeTrait, Destination:AlmanacTypeTrait> SourceToDestinationMap<Source, Destination> {
    fn convert(&self, source:Source) -> Destination {
        for range in &self.mapping_range_list {
            if range.is_source_in_range(source) {
                return range.convert(source);
            }
        }
        let sourceval = source.to_u32();
        return Destination::from_u32(sourceval);
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

