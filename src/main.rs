use itertools::Itertools;
use std::io;

fn main() {
    let test_result = solve("test.txt");
    assert_eq!(test_result, 46, "test input failed");

    let result = solve("input.txt");
    println!("result: {result}");
}

fn solve(path: &str) -> i64 {
    let input = Input::parse(path);

    let mut current_seeds = input.seeds;
    println!("initial seeds: {:?}", &current_seeds);

    for maps in &input.mappings {
        current_seeds = process_mapping(current_seeds, maps);
        println!("seeds after {}: {:?}", maps.name, &current_seeds);
    }

    current_seeds.iter().map(|i| i.start).min().unwrap()
}

fn process_mapping(mut input: Vec<SeedRange>, mapping: &SeedMapping) -> Vec<SeedRange> {
    let mut mapped_seeds = Vec::new();

    for map in &mapping.entries {
        let mut unmapped = Vec::new();

        for unprocessed in input {
            // maping will give a list of maybe unprocessed (before, after), and processed.
            // all 3 optional.
            let map_res = map_seed_range(unprocessed, map);

            if let Some(before) = map_res.before {
                unmapped.push(before);
            }

            if let Some(after) = map_res.after {
                unmapped.push(after);
            }

            if let Some(mapped) = map_res.mapped {
                mapped_seeds.push(mapped);
            }
        }

        // Any seeds, including partial seed ranges, that were not mapped need to be processed and
        // tried again on the next mapping entry
        //
        // It's important that we cosume all of 'input' at each stage, since it's possible one
        // mapping of a seed range may cause 2 unmapped regions for the next entry to process.
        //
        // This can happen if we have a particularly large seed range, say 100 to 200, and a small
        // mapping, say 'from' values of 110 to 115. The ranges left to map would be 100-110, and
        // 115-200.
        input = unmapped;
    }

    // Any seed ranges that wern't mapped at all stay 'as is', and are mapped by identity
    mapped_seeds.append(&mut input);
    mapped_seeds
}

struct MappingResult {
    before: Option<SeedRange>,
    after: Option<SeedRange>,
    mapped: Option<SeedRange>,
}

fn map_seed_range(seeds: SeedRange, mapping: &SeedMappingEntry) -> MappingResult {
    // Extract out any sections that are before / after mapping to their own new sub-ranges.
    let mut before = None;
    if seeds.start < mapping.source_start {
        let end = seeds.end().min(mapping.source_start);
        before = Some(SeedRange::from_start_end(seeds.start, end));
    }

    let mut after = None;
    if seeds.end() > mapping.source_end() {
        let start = seeds.start.max(mapping.source_end());
        after = Some(SeedRange::from_start_end(start, seeds.end()));
    }

    let mut mapped = None;
    if seeds.start < mapping.source_end() && seeds.end() > mapping.source_start {
        let start = seeds.start.max(mapping.source_start);
        let end = seeds.end().min(mapping.source_end());

        let map_offset = mapping.source_to_dest_offset();
        mapped = Some(SeedRange::from_start_end(
            start + map_offset,
            end + map_offset,
        ));
    }

    MappingResult {
        before,
        after,
        mapped,
    }
}

struct Input {
    seeds: Vec<SeedRange>,
    mappings: Vec<SeedMapping>,
}

impl Input {
    fn parse(path: &str) -> Self {
        // no Result return type, since out input from AOC should always be valid
        let mut input = read_lines(path).unwrap();

        let seeds: Vec<SeedRange> = {
            let seeds_str = input.next().unwrap().unwrap();
            let seeds_str = seeds_str.strip_prefix("seeds: ").unwrap();

            seeds_str
                .split_ascii_whitespace()
                .tuples()
                .map(|(a, b)| {
                    let start = a.parse::<i64>().unwrap();
                    let len = b.parse::<i64>().unwrap();
                    SeedRange { start, len }
                })
                .collect()
        };

        let mut mappings = Vec::new();

        for line in input {
            let line = line.as_ref().unwrap().trim();

            if line.is_empty() {
                continue;
            }

            if line.ends_with("map:") {
                // The beginning of a new seed mapping entry
                let mapping_name = line
                    .split_ascii_whitespace()
                    .next()
                    .expect("Got seed mapping start with no name");

                mappings.push(SeedMapping {
                    name: mapping_name.to_owned(),
                    entries: vec![],
                });
                continue;
            }

            // A new entry for the mapping we're currently working on
            let mapping = mappings
                .last_mut()
                .expect("Got mapping entry with no mapping start");

            mapping.entries.push(SeedMappingEntry::parse(line));
        }

        Self { seeds, mappings }
    }
}

#[derive(Clone, Copy, Debug)]
struct SeedRange {
    start: i64,
    len: i64,
}

impl SeedRange {
    // one past end (start/end is a half open range)
    fn end(self) -> i64 {
        self.start + self.len
    }

    fn from_start_end(start: i64, end: i64) -> SeedRange {
        assert!(start < end);
        SeedRange {
            start,
            len: end - start,
        }
    }
}

struct SeedMapping {
    name: String,
    entries: Vec<SeedMappingEntry>,
}

#[derive(Clone, Copy)]
struct SeedMappingEntry {
    dest_start: i64,
    source_start: i64,
    len: i64,
}

impl SeedMappingEntry {
    fn parse(line: &str) -> Self {
        // WARNING: The line in the almanac has destination _first_, then source.
        let entries = line.split_ascii_whitespace().collect_tuple().unwrap();
        let (dest, source, len) = entries;

        SeedMappingEntry {
            dest_start: dest.parse().unwrap(),
            source_start: source.parse().unwrap(),
            len: len.parse().unwrap(),
        }
    }

    // one past end (start/end is a half open range)
    fn source_end(&self) -> i64 {
        self.source_start + self.len
    }

    fn source_to_dest_offset(&self) -> i64 {
        self.dest_start - self.source_start
    }
}
fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<std::fs::File>>> {
    let file = std::fs::File::open(filename)?;
    Ok(io::BufRead::lines(io::BufReader::new(file)))
}
