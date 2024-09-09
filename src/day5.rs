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

#[derive(PartialEq,Debug,Clone,Copy)]
struct Seed(u64);

#[derive(PartialEq,Debug,Clone,Copy)]
struct Soil(u64);

#[derive(PartialEq,Debug,Clone,Copy)]
struct Fertilizer(u64);

#[derive(PartialEq,Debug,Clone,Copy)]
struct Water(u64);

#[derive(PartialEq,Debug,Clone,Copy)]
struct Light(u64);

#[derive(PartialEq,Debug,Clone,Copy)]
struct Temperature(u64);

#[derive(PartialEq,Debug,Clone,Copy)]
struct Humidity(u64);

#[derive(PartialEq,Debug,Clone,Copy,Eq,PartialOrd,Ord)]
struct Location(u64);

trait AlmanacTypeTrait {
    fn to_u64(&self) -> u64;
    fn from_u64(value:u64) -> Self;
}

impl AlmanacTypeTrait for Seed {
    fn to_u64(&self) -> u64 { self.0 }
    fn from_u64(value:u64) -> Self { Self(value )}
}

impl AlmanacTypeTrait for Soil {
    fn to_u64(&self) -> u64 { self.0 }
    fn from_u64(value:u64) -> Self { Self(value )}
}

impl AlmanacTypeTrait for Fertilizer {
    fn to_u64(&self) -> u64 { self.0 }
    fn from_u64(value:u64) -> Self { Self(value )}
}

impl AlmanacTypeTrait for Water {
    fn to_u64(&self) -> u64 { self.0 }
    fn from_u64(value:u64) -> Self { Self(value )}
}

impl AlmanacTypeTrait for Light {
    fn to_u64(&self) -> u64 { self.0 }
    fn from_u64(value:u64) -> Self { Self(value )}
}

impl AlmanacTypeTrait for Temperature {
    fn to_u64(&self) -> u64 { self.0 }
    fn from_u64(value:u64) -> Self { Self(value )}
}

impl AlmanacTypeTrait for Humidity {
    fn to_u64(&self) -> u64 { self.0 }
    fn from_u64(value:u64) -> Self { Self(value )}
}

impl AlmanacTypeTrait for Location {
    fn to_u64(&self) -> u64 { self.0 }
    fn from_u64(value:u64) -> Self { Self(value )}
}

struct MappingRange<Destination:AlmanacTypeTrait, Source:AlmanacTypeTrait> {
    destination_range_start: Destination,
    source_range_start: Source,
    range_length: u64
}

impl<Destination:AlmanacTypeTrait, Source:AlmanacTypeTrait> MappingRange<Destination, Source> {
    fn is_source_in_range(&self, source:Source) -> bool {
        source.to_u64() >= self.source_range_start.to_u64()
        &&
        source.to_u64() < self.source_range_start.to_u64() + self.range_length
    }

    fn convert(&self, source:Source) -> Destination {
        Destination::from_u64(
            self.destination_range_start.to_u64() +
            ( source.to_u64() - self.source_range_start.to_u64() ))
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

struct SourceToDestinationMap<Source:AlmanacTypeTrait, Destination:AlmanacTypeTrait> {
    mapping_range_list:Vec<MappingRange<Destination, Source>>
}

impl<Source:AlmanacTypeTrait+Copy, Destination:AlmanacTypeTrait+Copy> SourceToDestinationMap<Source, Destination> {
    fn new() -> Self {
        SourceToDestinationMap { mapping_range_list:Vec::new() }
    }
    fn add_range(&mut self, source_range_start: Source, destination_range_start: Destination, range_length: u64) -> () {
        self.mapping_range_list.push(MappingRange {
            source_range_start : source_range_start,
            destination_range_start : destination_range_start,
            range_length : range_length
        });
    }
    fn convert(&self, source:Source) -> Destination {
        for range in &self.mapping_range_list {
            if range.is_source_in_range(source) {
                return range.convert(source);
            }
        }
        let sourceval = source.to_u64();
        return Destination::from_u64(sourceval);
    }
    fn convert_vector(&self, source:&Vec<Source>) -> Vec<Destination> {
        source.into_iter().map(|source| self.convert(*source)).collect()
    }
}

struct Almanac {
    seeds: Vec<Seed>,
    seed_to_soil: SourceToDestinationMap<Seed, Soil>,
    soil_to_fertilizer: SourceToDestinationMap<Soil, Fertilizer>,
    fertilizer_to_water: SourceToDestinationMap<Fertilizer, Water>,
    water_to_light: SourceToDestinationMap<Water, Light>,
    light_to_temperature: SourceToDestinationMap<Light, Temperature>,
    temperature_to_humidity: SourceToDestinationMap<Temperature, Humidity>,
    humidity_to_location: SourceToDestinationMap<Humidity, Location>
}

impl Almanac {
    fn new() -> Almanac {
        {
            Almanac {
                seeds:Vec::new(),
                seed_to_soil: SourceToDestinationMap::new(),
                soil_to_fertilizer: SourceToDestinationMap::new(),
                fertilizer_to_water: SourceToDestinationMap::new(),
                water_to_light: SourceToDestinationMap::new(),
                light_to_temperature: SourceToDestinationMap::new(),
                temperature_to_humidity: SourceToDestinationMap::new(),
                humidity_to_location: SourceToDestinationMap::new()
            }
        }
    }
}

use pest::iterators::Pair;

fn build_source_destination_map<Source:AlmanacTypeTrait+Copy, Destination:AlmanacTypeTrait+Copy>
    (mapping_rule:Pair<'_, Rule>) -> SourceToDestinationMap<Source, Destination> {
        let mut sd_map = SourceToDestinationMap::new();
        for list_of_triples in mapping_rule.into_inner() {
            match list_of_triples.as_rule() {
                Rule::list_of_triples => {
                    let mut number_iter = list_of_triples.into_inner();
                    while let Some(destination_rule) = number_iter.next() {
                        let destination_value = destination_rule.as_str().parse::<u64>().unwrap();

                        let source_rule = number_iter.next().unwrap();
                        let source_value = source_rule.as_str().parse::<u64>().unwrap();

                        let range_rule = number_iter.next().unwrap();
                        let range_value = range_rule.as_str().parse::<u64>().unwrap();

                        sd_map.add_range(Source::from_u64(source_value), Destination::from_u64(destination_value), range_value);
                    }
                }
                _ => { println!("Unexpected {}", list_of_triples); }
            }
        }
        sd_map
}

#[derive(Debug, Clone, Copy)]
enum BuildAlmanacMode {
    Part1,
    Part2
}

// 79 14 55 13 = [79, 14, 55, 13]
fn build_seeds1(seeds_rule:Pair<'_, Rule>) -> Vec<Seed> {
    let mut seeds = Vec::new();
    for number in seeds_rule.into_inner() {
        match number.as_rule() {
            Rule::number => {
                let number_value = number.as_str().parse::<u64>().unwrap();
                seeds.push(Seed(number_value));
            }
            _ => { println!("Unexpected {}", number); }
        }
    }
    seeds
}

// 79 14 55 13 = [79..79+14, 55.. 55+13]
fn build_seeds2(seeds_rule:Pair<'_, Rule>) -> Vec<Seed> {
    let mut seeds = Vec::new();
    let mut number_iter = seeds_rule.into_inner();
    while let Some(seed_start_number_rule) = number_iter.next() {
        let seed_start_number_value = seed_start_number_rule.as_str().parse::<u64>().unwrap();

        let range_rule = number_iter.next().unwrap();
        let range_value = range_rule.as_str().parse::<u64>().unwrap();
        println!("       Seed range: {}..{}",seed_start_number_value, seed_start_number_value + range_value);
        for seed_number_value in seed_start_number_value .. seed_start_number_value + range_value {
            seeds.push(Seed::from_u64(seed_number_value));
        }
    }
    seeds
}

fn build_almanac(file_rule:Pair<'_, Rule>, mode: BuildAlmanacMode) -> Almanac {
    let mut almanac:Almanac = Almanac::new();
    for almanac_entry in file_rule.into_inner() {
        match almanac_entry.as_rule() {
            Rule::seeds => {
                almanac.seeds = match mode {
                    BuildAlmanacMode::Part1 => build_seeds1(almanac_entry),
                    BuildAlmanacMode::Part2 => build_seeds2(almanac_entry)
                };
            },
            Rule::seed_to_soil => {
                almanac.seed_to_soil = build_source_destination_map(almanac_entry);
            },
            Rule::soil_to_fertilizer => {
                almanac.soil_to_fertilizer = build_source_destination_map(almanac_entry);
            },
            Rule::fertilizer_to_water => {
                almanac.fertilizer_to_water = build_source_destination_map(almanac_entry);
            },
            Rule::water_to_light => {
                almanac.water_to_light = build_source_destination_map(almanac_entry);
            },
            Rule::light_to_temperature => {
                almanac.light_to_temperature = build_source_destination_map(almanac_entry);
            },
            Rule::temperature_to_humidity => {
                almanac.temperature_to_humidity = build_source_destination_map(almanac_entry);
            },
            Rule::humidity_to_location => {
                almanac.humidity_to_location = build_source_destination_map(almanac_entry);
            },
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }
    almanac
}

#[cfg(test)]
fn build_example_almanac(mode: BuildAlmanacMode) -> Almanac {
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
    let concat_input = input.join("\n");
    let mut parsed = Day5Parser::parse(Rule::file, &concat_input).unwrap();
    let file_rule = parsed.next().unwrap();
    build_almanac(file_rule, mode)
}

#[test]
fn test_example1() {
    let almanac = build_example_almanac(BuildAlmanacMode::Part1);

    assert_eq!(almanac.seeds, vec![Seed(79), Seed(14), Seed(55), Seed(13)]);
    assert_eq!(almanac.seed_to_soil.mapping_range_list.len(), 2);
    assert_eq!(almanac.soil_to_fertilizer.mapping_range_list.len(), 3);
    assert_eq!(almanac.fertilizer_to_water.mapping_range_list.len(), 4);
    assert_eq!(almanac.water_to_light.mapping_range_list.len(), 2);
    assert_eq!(almanac.light_to_temperature.mapping_range_list.len(), 3);
    assert_eq!(almanac.temperature_to_humidity.mapping_range_list.len(), 2);
    assert_eq!(almanac.humidity_to_location.mapping_range_list.len(), 2);

    let soils = almanac.seed_to_soil.convert_vector(&almanac.seeds);
    assert_eq!(soils, vec![Soil(81), Soil(14), Soil(57), Soil(13)]);

    let fertilizers = almanac.soil_to_fertilizer.convert_vector(&soils);
    assert_eq!(fertilizers, vec![Fertilizer(81), Fertilizer(53), Fertilizer(57), Fertilizer(52)]);

    let water = almanac.fertilizer_to_water.convert_vector(&fertilizers);
    assert_eq!(water, vec![Water(81), Water(49), Water(53), Water(41)]);

    let lights = almanac.water_to_light.convert_vector(&water);
    assert_eq!(lights, vec![Light(74), Light(42), Light(46), Light(34)]);

    let temperatures = almanac.light_to_temperature.convert_vector(&lights);
    assert_eq!(temperatures, vec![Temperature(78), Temperature(42), Temperature(82), Temperature(34)]);

    let humidities = almanac.temperature_to_humidity.convert_vector(&temperatures);
    assert_eq!(humidities, vec![Humidity(78), Humidity(43), Humidity(82), Humidity(35)]);

    let locations = almanac.humidity_to_location.convert_vector(&humidities);
    assert_eq!(locations, vec![Location(82), Location(43), Location(86), Location(35)]);

    let lowest_location = locations.iter().min().unwrap();
    assert_eq!(lowest_location, &Location(35));

}

#[test]
fn test_example2() {
    let almanac = build_example_almanac(BuildAlmanacMode::Part2);

    //Beginners way ;-)
    //let mut seed_val_exp:Vec<u64> = Vec::new();
    //seed_val_exp.extend(79..79+14);
    //seed_val_exp.extend(55..55+13);
    //let seed_exp:Vec<Seed> = seed_val_exp.into_iter().map(|x| Seed(x)).collect();
    let seed_exp:Vec<Seed> = (79..79+14).chain(55..55+13).map(|x| Seed(x)).collect();
    assert_eq!(almanac.seeds, seed_exp);
    assert_eq!(almanac.seed_to_soil.mapping_range_list.len(), 2);
    assert_eq!(almanac.soil_to_fertilizer.mapping_range_list.len(), 3);
    assert_eq!(almanac.fertilizer_to_water.mapping_range_list.len(), 4);
    assert_eq!(almanac.water_to_light.mapping_range_list.len(), 2);
    assert_eq!(almanac.light_to_temperature.mapping_range_list.len(), 3);
    assert_eq!(almanac.temperature_to_humidity.mapping_range_list.len(), 2);
    assert_eq!(almanac.humidity_to_location.mapping_range_list.len(), 2);

    let soils = almanac.seed_to_soil.convert_vector(&almanac.seeds);
    assert_eq!(soils[3], Soil(84));

    let fertilizers = almanac.soil_to_fertilizer.convert_vector(&soils);
    assert_eq!(fertilizers[3], Fertilizer(84));

    let water = almanac.fertilizer_to_water.convert_vector(&fertilizers);
    assert_eq!(water[3], Water(84));

    let lights = almanac.water_to_light.convert_vector(&water);
    assert_eq!(lights[3], Light(77));

    let temperatures = almanac.light_to_temperature.convert_vector(&lights);
    assert_eq!(temperatures[3], Temperature(45));

    let humidities = almanac.temperature_to_humidity.convert_vector(&temperatures);
    assert_eq!(humidities[3], Humidity(46));

    let locations = almanac.humidity_to_location.convert_vector(&humidities);
    assert_eq!(locations[3], Location(46));

    let lowest_location = locations.iter().min().unwrap();
    assert_eq!(lowest_location, &Location(46));

}

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn part1and2() {

    for mode in [BuildAlmanacMode::Part1, BuildAlmanacMode::Part2] {
        let file = File::open("data/day5.input").expect("Could not open data/day5.input");
        let reader = BufReader::new(file);

        let lines:Vec<String> = reader.lines().map( |line| line.unwrap() ).collect();
        let concat_input = lines.join("\n");
        let mut parsed = Day5Parser::parse(Rule::file, &concat_input).unwrap();
        let file_rule = parsed.next().unwrap();
        let almanac = build_almanac(file_rule, mode);

        let soils = almanac.seed_to_soil.convert_vector(&almanac.seeds);
        let fertilizers = almanac.soil_to_fertilizer.convert_vector(&soils);
        let water = almanac.fertilizer_to_water.convert_vector(&fertilizers);
        let lights = almanac.water_to_light.convert_vector(&water);
        let temperatures = almanac.light_to_temperature.convert_vector(&lights);
        let humidities = almanac.temperature_to_humidity.convert_vector(&temperatures);
        let locations = almanac.humidity_to_location.convert_vector(&humidities);
        let lowest_location = locations.iter().min().unwrap();

        println!("Day 5, {:#?}: Lowest location is {}", mode, lowest_location.to_u64());
    }
}
