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

#[derive(PartialEq,Debug,Clone,Copy,Eq,PartialOrd,Ord)]
struct Seed(u64);

#[derive(PartialEq,Debug,Clone,Copy,Eq,PartialOrd,Ord)]
struct Soil(u64);

#[derive(PartialEq,Debug,Clone,Copy,Eq,PartialOrd,Ord)]
struct Fertilizer(u64);

#[derive(PartialEq,Debug,Clone,Copy,Eq,PartialOrd,Ord)]
struct Water(u64);

#[derive(PartialEq,Debug,Clone,Copy,Eq,PartialOrd,Ord)]
struct Light(u64);

#[derive(PartialEq,Debug,Clone,Copy,Eq,PartialOrd,Ord)]
struct Temperature(u64);

#[derive(PartialEq,Debug,Clone,Copy,Eq,PartialOrd,Ord)]
struct Humidity(u64);

#[derive(PartialEq,Debug,Clone,Copy,Eq,PartialOrd,Ord)]
struct Location(u64);

trait AlmanacType: Copy+Ord{
    fn to_u64(&self) -> u64;
    fn from_u64(value:u64) -> Self;
}

impl AlmanacType for Seed {
    fn to_u64(&self) -> u64 { self.0 }
    fn from_u64(value:u64) -> Self { Self(value )}
}

impl AlmanacType for Soil {
    fn to_u64(&self) -> u64 { self.0 }
    fn from_u64(value:u64) -> Self { Self(value )}
}

impl AlmanacType for Fertilizer {
    fn to_u64(&self) -> u64 { self.0 }
    fn from_u64(value:u64) -> Self { Self(value )}
}

impl AlmanacType for Water {
    fn to_u64(&self) -> u64 { self.0 }
    fn from_u64(value:u64) -> Self { Self(value )}
}

impl AlmanacType for Light {
    fn to_u64(&self) -> u64 { self.0 }
    fn from_u64(value:u64) -> Self { Self(value )}
}

impl AlmanacType for Temperature {
    fn to_u64(&self) -> u64 { self.0 }
    fn from_u64(value:u64) -> Self { Self(value )}
}

impl AlmanacType for Humidity {
    fn to_u64(&self) -> u64 { self.0 }
    fn from_u64(value:u64) -> Self { Self(value )}
}

impl AlmanacType for Location {
    fn to_u64(&self) -> u64 { self.0 }
    fn from_u64(value:u64) -> Self { Self(value )}
}

struct MappingRange<Destination:AlmanacType, Source:AlmanacType> {
    destination_range_start: Destination,
    source_range_start: Source,
    range_length: u64
}

use std::cmp::min;
use std::cmp::max;

#[derive(PartialEq,Debug)]
struct MappingRangeConversionResult<Destination:AlmanacType, Source:AlmanacType> {
    before: Option<Range<Source>>,
    mapped: Option<Range<Destination>>,
    behind: Option<Range<Source>>
}

impl<Destination:AlmanacType, Source:AlmanacType> MappingRange<Destination, Source> {
    fn is_source_in_range(&self, source:Source) -> bool {
        source >= self.source_range_start
        &&
        source < Source::from_u64(self.source_range_start.to_u64() + self.range_length)
    }

    fn convert(&self, source:Source) -> Destination {
        Destination::from_u64(
            self.destination_range_start.to_u64() +
            ( source.to_u64() - self.source_range_start.to_u64() ))
    }

    // convert a..b according to mapping range
    // result can be up to three ranges:
    //           |----MappingSourceRange--|
    //   |-------------sourceRange------------------|
    //   |-before|                        |-behind--|     |---destinationRange---|
    fn convert_range(&self, source_range:&Range<Source>) -> MappingRangeConversionResult<Destination, Source> {
        let mapping_start = self.source_range_start;
        let mapping_end = Source::from_u64(self.source_range_start.to_u64() + self.range_length);
        MappingRangeConversionResult {
            before: if source_range.start < mapping_start { Some(source_range.start .. min(source_range.end, mapping_start))} else { None },
            behind: if source_range.end   > mapping_end   { Some(max(source_range.start, mapping_end) .. source_range.end)} else { None },
            mapped: if source_range.start < mapping_end && source_range.end >= mapping_start
                      { Some(self.convert(max(source_range.start, mapping_start)) .. self.convert(min(source_range.end, mapping_end))) }
                    else { None }
        }
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

    assert_eq!(range.convert_range(&(Seed(50)..Seed(55))),
      MappingRangeConversionResult {
        before: Some(Seed(50)..Seed(55)),
        mapped: None,
        behind: None
      });

      assert_eq!(range.convert_range(&(Seed(150)..Seed(155))),
      MappingRangeConversionResult {
        before: None,
        mapped: None,
        behind: Some(Seed(150)..Seed(155))
      });

      assert_eq!(range.convert_range(&(Seed(98)..Seed(100))),
      MappingRangeConversionResult {
        before: None,
        mapped: Some(Soil(50)..Soil(52)),
        behind: None
      });

      assert_eq!(range.convert_range(&(Seed(90)..Seed(105))),
      MappingRangeConversionResult {
        before: Some(Seed(90)..Seed(98)),
        mapped: Some(Soil(50)..Soil(52)),
        behind: Some(Seed(100)..Seed(105))
      });

}


use std::ops::Range;
// A list of ranges, e.g. [3..5, 7..9, 11..12] = [3,4,7,8,11]
#[derive(Clone)]
struct RangeList<T:AlmanacType> {
    ranges: Vec<Range<T>>
}

impl<T:AlmanacType> RangeList<T> {
    // create single-valued ranges: [3,5,11] -> [3..4, 5..6, 11..12]
    fn create_single_valued_ranges(single_values: &Vec<T>) -> Self {
        let mut vec:Vec<Range<T>> = Vec::new();
        for t in single_values {
            vec.push(*t .. T::from_u64(t.to_u64()+1));
        }
        Self { ranges: vec }
    }

    // create real ranges: [(3,5),(7,9),(11,12)] -> [3..5, 7..9, 11..12]
    fn create_real_ranges(ranges: &Vec<Range<T>>) -> Self {
        Self { ranges: ranges.clone() }
    }

    fn new() -> Self {
        Self { ranges: Vec::new() }
    }

    fn to_vec(&self) -> Vec<T> {
        let mut vec = Vec::new();
        for range in &self.ranges {
            for value in range.start.to_u64()..range.end.to_u64() {
                vec.push(T::from_u64(value));
            }
        }
        vec
    }

    fn to_sorted_vec(&self) -> Vec<T> {
        let mut vec = self.to_vec();
        vec.sort_unstable();
        vec
    }

    fn min(&self) -> Option<T> {
        let mut current_min = None;
        for range in &self.ranges {
            current_min = match current_min {
              None => Some(range.start),
              Some(current) => Some(min(current, range.start))
            }
//            if current_min.is_none() {
//                current_min = Some(range.start);
//            } else {
//                current_min = Some(min(current_min, range.start));
//            }
        }

        current_min
    }

}

#[test]
fn test_range_list() {
    let range_list1 = RangeList::create_single_valued_ranges(&[Seed(3), Seed(5), Seed(11)].to_vec());
    assert_eq!(range_list1.ranges, vec![Seed(3)..Seed(4), Seed(5)..Seed(6), Seed(11)..Seed(12)]);
    assert_eq!(range_list1.to_vec(), vec![Seed(3), Seed(5), Seed(11)]);

    let range_list2 = RangeList::create_real_ranges(&[Seed(3)..Seed(5), Seed(7)..Seed(9), Seed(11)..Seed(12)].to_vec());
    assert_eq!(range_list2.ranges, vec![Seed(3)..Seed(5), Seed(7)..Seed(9), Seed(11)..Seed(12)]);
    assert_eq!(range_list2.to_vec(), vec![Seed(3),Seed(4), Seed(7), Seed(8), Seed(11)]);

    let range_list3 = RangeList::create_real_ranges(&[Seed(8)..Seed(12), Seed(4)..Seed(5), Seed(11)..Seed(12)].to_vec());
    assert_eq!(range_list3.min().unwrap(), Seed(4));
}


struct SourceToDestinationMap<Source:AlmanacType, Destination:AlmanacType> {
    mapping_range_list:Vec<MappingRange<Destination, Source>>
}

impl<Source:AlmanacType, Destination:AlmanacType> SourceToDestinationMap<Source, Destination> {
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
        return Destination::from_u64(source.to_u64());
    }

    fn convert_range_list(&self, source_range_list:&RangeList<Source>) -> RangeList<Destination> {
        let mut ranges_to_convert:RangeList<Source> = source_range_list.clone();
        let mut ranges_to_convert_next = RangeList::<Source>::new();
        let mut ranges_converted = RangeList::<Destination>::new();
        for mapping_range in &self.mapping_range_list {
            for source_range in &ranges_to_convert.ranges {
                let conversion_result = mapping_range.convert_range(&source_range);
                if conversion_result.before.is_some() {
                    ranges_to_convert_next.ranges.push(conversion_result.before.unwrap());
                }
                if conversion_result.behind.is_some() {
                    ranges_to_convert_next.ranges.push(conversion_result.behind.unwrap());
                }
                if conversion_result.mapped.is_some() {
                    ranges_converted.ranges.push(conversion_result.mapped.unwrap());
                }
            }
            ranges_to_convert = ranges_to_convert_next;
            ranges_to_convert_next = RangeList::new();
        }

        // now use identity transformation for the not-yet-converted values
        for source_range in &ranges_to_convert.ranges {
            ranges_converted.ranges.push(Destination::from_u64(source_range.start.to_u64()) .. Destination::from_u64(source_range.end.to_u64()));
        }

        ranges_converted
    }

}

#[test]
fn test_convert_range_list() {
    //  4..6 -> 14..16, 7..9 -> 27..29
    let mappings = {
        let mut mappings = SourceToDestinationMap::<Seed, Soil>::new();
        mappings.add_range(Seed(4),Soil(14),2);
        mappings.add_range(Seed(7),Soil(27),2);
        mappings
    };

    let seeds = RangeList::create_real_ranges(&vec![Seed(0)..Seed(10)]);
    let converted = mappings.convert_range_list(&seeds);

    assert_eq!(converted.ranges,
        vec![Soil(14)..Soil(16),
             Soil(27)..Soil(29),
             Soil(0)..Soil(4),
             Soil(6)..Soil(7),
             Soil(9)..Soil(10)]);

}

struct Almanac {
    seeds: RangeList<Seed>,
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
                seeds:RangeList::new(),
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

impl Seed        { fn            seed_to_soil(&self, almanac:&Almanac) -> Soil        {            almanac.seed_to_soil.convert(*self) } }
impl Soil        { fn      soil_to_fertilizer(&self, almanac:&Almanac) -> Fertilizer  {      almanac.soil_to_fertilizer.convert(*self) } }
impl Fertilizer  { fn     fertilizer_to_water(&self, almanac:&Almanac) -> Water       {     almanac.fertilizer_to_water.convert(*self) } }
impl Water       { fn          water_to_light(&self, almanac:&Almanac) -> Light       {          almanac.water_to_light.convert(*self) } }
impl Light       { fn    light_to_temperature(&self, almanac:&Almanac) -> Temperature {    almanac.light_to_temperature.convert(*self) } }
impl Temperature { fn temperature_to_humidity(&self, almanac:&Almanac) -> Humidity    { almanac.temperature_to_humidity.convert(*self) } }
impl Humidity    { fn    humidity_to_location(&self, almanac:&Almanac) -> Location    {    almanac.humidity_to_location.convert(*self) } }

use pest::iterators::Pair;

fn build_source_destination_map<Source:AlmanacType, Destination:AlmanacType>
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
fn build_seeds1(seeds_rule:Pair<'_, Rule>) -> RangeList<Seed> {
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
    RangeList::create_single_valued_ranges(&seeds)
}

// 79 14 55 13 = [79..79+14, 55.. 55+13]
fn build_seeds2(seeds_rule:Pair<'_, Rule>) -> RangeList<Seed> {
    let mut ranges = Vec::new();
    let mut number_iter = seeds_rule.into_inner();
    while let Some(seed_start_number_rule) = number_iter.next() {
        let seed_start_number_value = seed_start_number_rule.as_str().parse::<u64>().unwrap();

        let range_rule = number_iter.next().unwrap();
        let range_value = range_rule.as_str().parse::<u64>().unwrap();
        //println!("       Seed range: {}..{}",seed_start_number_value, seed_start_number_value + range_value);
        ranges.push(Seed::from_u64(seed_start_number_value)..Seed::from_u64(seed_start_number_value + range_value));
    }
    RangeList::create_real_ranges(&ranges)
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

    assert_eq!(&almanac.seeds.ranges, &vec![Seed(79)..Seed(80), Seed(14)..Seed(15), Seed(55)..Seed(56), Seed(13)..Seed(14)]);
    assert_eq!(almanac.seed_to_soil.mapping_range_list.len(), 2);
    assert_eq!(almanac.soil_to_fertilizer.mapping_range_list.len(), 3);
    assert_eq!(almanac.fertilizer_to_water.mapping_range_list.len(), 4);
    assert_eq!(almanac.water_to_light.mapping_range_list.len(), 2);
    assert_eq!(almanac.light_to_temperature.mapping_range_list.len(), 3);
    assert_eq!(almanac.temperature_to_humidity.mapping_range_list.len(), 2);
    assert_eq!(almanac.humidity_to_location.mapping_range_list.len(), 2);

    let soils = almanac.seed_to_soil.convert_range_list(&almanac.seeds);
//    assert_eq!(soils.to_vec(), vec![Soil(81), Soil(57), Soil(14), Soil(13)]);
    assert_eq!(soils.to_sorted_vec(), vec![Soil(13), Soil(14), Soil(57), Soil(81)]);

    let fertilizers = almanac.soil_to_fertilizer.convert_range_list(&soils);
    assert_eq!(fertilizers.to_sorted_vec(), vec![Fertilizer(52), Fertilizer(53), Fertilizer(57), Fertilizer(81)]);

    let water = almanac.fertilizer_to_water.convert_range_list(&fertilizers);
    assert_eq!(water.to_sorted_vec(), vec![Water(41), Water(49), Water(53), Water(81)]);

    let lights = almanac.water_to_light.convert_range_list(&water);
    assert_eq!(lights.to_sorted_vec(), vec![Light(34), Light(42), Light(46), Light(74)]);

    let temperatures = almanac.light_to_temperature.convert_range_list(&lights);
    assert_eq!(temperatures.to_sorted_vec(), vec![Temperature(34), Temperature(42), Temperature(78), Temperature(82)]);

    let humidities = almanac.temperature_to_humidity.convert_range_list(&temperatures);
    assert_eq!(humidities.to_sorted_vec(), vec![Humidity(35), Humidity(43), Humidity(78), Humidity(82)]);

    let locations = almanac.humidity_to_location.convert_range_list(&humidities);
    assert_eq!(locations.to_sorted_vec(), vec![Location(35), Location(43), Location(82), Location(86)]);

    let lowest_location = locations.min().unwrap();
    assert_eq!(lowest_location, Location(35));

}

#[test]
fn test_example2() {
    let almanac = build_example_almanac(BuildAlmanacMode::Part2);

    //Beginners way ;-)
    //let mut seed_val_exp:Vec<u64> = Vec::new();
    //seed_val_exp.extend(79..79+14);
    //seed_val_exp.extend(55..55+13);
    //let seed_exp:Vec<Seed> = seed_val_exp.iter().map(|x| Seed(x)).collect();
    let seed_exp:Vec<Seed> = (79..79+14).chain(55..55+13).map(|x| Seed(x)).collect();
    assert_eq!(almanac.seeds.to_vec(), seed_exp);
    assert_eq!(almanac.seed_to_soil.mapping_range_list.len(), 2);
    assert_eq!(almanac.soil_to_fertilizer.mapping_range_list.len(), 3);
    assert_eq!(almanac.fertilizer_to_water.mapping_range_list.len(), 4);
    assert_eq!(almanac.water_to_light.mapping_range_list.len(), 2);
    assert_eq!(almanac.light_to_temperature.mapping_range_list.len(), 3);
    assert_eq!(almanac.temperature_to_humidity.mapping_range_list.len(), 2);
    assert_eq!(almanac.humidity_to_location.mapping_range_list.len(), 2);

    // check fourth value
    let seed = almanac.seeds.to_vec()[3];
    assert_eq!(seed, Seed(82));

    let soil = seed.seed_to_soil(&almanac);
    assert_eq!(soil, Soil(84));

    let fertilizer = soil.soil_to_fertilizer(&almanac);
    assert_eq!(fertilizer, Fertilizer(84));

    let water = fertilizer.fertilizer_to_water(&almanac);
    assert_eq!(water, Water(84));

    let light = water.water_to_light(&almanac);
    assert_eq!(light, Light(77));

    let temperature = light.light_to_temperature(&almanac);
    assert_eq!(temperature, Temperature(45));

    let humidity = temperature.temperature_to_humidity(&almanac);
    assert_eq!(humidity, Humidity(46));

    let location = humidity.humidity_to_location(&almanac);
    assert_eq!(location, Location(46));

    let soils = almanac.seed_to_soil.convert_range_list(&almanac.seeds);
    let fertilizers = almanac.soil_to_fertilizer.convert_range_list(&soils);
    let water = almanac.fertilizer_to_water.convert_range_list(&fertilizers);
    let lights = almanac.water_to_light.convert_range_list(&water);
    let temperatures = almanac.light_to_temperature.convert_range_list(&lights);
    let humidities = almanac.temperature_to_humidity.convert_range_list(&temperatures);
    let locations = almanac.humidity_to_location.convert_range_list(&humidities);
    let lowest_location = locations.min().unwrap();
    assert_eq!(lowest_location, Location(46));

}

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::time::Instant;

pub fn part1and2() {

    for mode in [BuildAlmanacMode::Part1, BuildAlmanacMode::Part2] {
        let file = File::open("data/day5.input").expect("Could not open data/day5.input");
        let reader = BufReader::new(file);

        let lines:Vec<String> = reader.lines().map( |line| line.unwrap() ).collect();
        let concat_input = lines.join("\n");
        let mut parsed = Day5Parser::parse(Rule::file, &concat_input).unwrap();
        let file_rule = parsed.next().unwrap();
        let almanac = build_almanac(file_rule, mode);

        let start = Instant::now();

        println!("Day 5, {:#?}: Number of seed ranges is {} ({} seconds)", mode, almanac.seeds.ranges.len(), start.elapsed().as_secs());

        let soils = almanac.seed_to_soil.convert_range_list(&almanac.seeds);

        println!("Day 5, {:#?}: Number of soil ranges is {} ({} seconds)", mode, soils.ranges.len(), start.elapsed().as_secs());

        let fertilizers = almanac.soil_to_fertilizer.convert_range_list(&soils);
        let water = almanac.fertilizer_to_water.convert_range_list(&fertilizers);
        let lights = almanac.water_to_light.convert_range_list(&water);
        let temperatures = almanac.light_to_temperature.convert_range_list(&lights);
        let humidities = almanac.temperature_to_humidity.convert_range_list(&temperatures);
        let locations = almanac.humidity_to_location.convert_range_list(&humidities);

        println!("Day 5, {:#?}: Number of location ranges is {} ({} seconds)", mode, locations.ranges.len(), start.elapsed().as_secs());

        let lowest_location = locations.min().unwrap();

        println!("Day 5, {:#?}: Lowest location is {} ({} seconds)", mode, lowest_location.to_u64(), start.elapsed().as_secs());
    }
}
