#[derive(Debug)]
struct RangeMapEntry {
    dest_start: usize,
    src_start: usize,
    length: usize,
}

impl RangeMapEntry {
    pub fn try_map_num(&self, num: usize) -> Option<usize> {
        (self.src_start..(self.src_start + self.length))
            .contains(&num)
            .then(|| num - self.src_start + self.dest_start)
    }

    pub fn try_map_num_rev(&self, num: usize) -> Option<usize> {
        (self.dest_start..(self.dest_start + self.length))
            .contains(&num)
            .then(|| num - self.dest_start + self.src_start)
    }

    pub fn from_line(line: &str) -> Self {
        let mut split = line.split_ascii_whitespace().map(|s| s.parse().unwrap());
        let dest_start = split.next().unwrap();
        let src_start = split.next().unwrap();
        let length = split.next().unwrap();
        RangeMapEntry {
            dest_start,
            src_start,
            length,
        }
    }
}

#[derive(Debug)]
struct RangeMap {
    entries: Vec<RangeMapEntry>,
}

impl RangeMap {
    pub fn map_num(&self, num: usize) -> usize {
        self.entries
            .iter()
            .find_map(|entry| entry.try_map_num(num))
            .unwrap_or(num)
    }
    pub fn map_num_rev(&self, num: usize) -> usize {
        self.entries
            .iter()
            .find_map(|entry| entry.try_map_num_rev(num))
            .unwrap_or(num)
    }
}

#[allow(dead_code)]
fn get_demo_input() -> &'static str {
    "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
}

#[allow(dead_code)]
fn part2_simple(seeds: &[usize], range_maps: &[RangeMap]) {
    let seeds_ranges: Vec<_> = seeds
        .chunks_exact(2)
        .map(|chunk| (chunk[0], chunk[1]))
        .collect();
    // let's just brute force it :)
    let out2 = seeds_ranges
        .iter()
        .flat_map(|(start, length)| {
            println!("{start},{length}");
            (*start..(*start + *length)).into_iter()
        })
        .map(|seed| {
            range_maps
                .iter()
                .fold(seed, |state, range_map| range_map.map_num(state))
        })
        .min();
    println!("{}", out2.unwrap());
}

pub fn main() {
    let input = include_str!("../input/05.txt");
    // let input = get_demo_input();
    let mut lines = input.lines().filter(|l| !l.is_empty()).peekable();
    let seed_line = lines.next().unwrap();
    let seeds: Vec<usize> = seed_line[7..]
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let _ = lines.next(); // skip first map line
    let mut range_maps = Vec::new();
    let mut current_range_map = RangeMap {
        entries: Vec::new(),
    };
    while let Some(line) = lines.next() {
        if !line.as_bytes()[0].is_ascii_digit() {
            range_maps.push(current_range_map);
            current_range_map = RangeMap {
                entries: Vec::new(),
            };
        } else {
            current_range_map
                .entries
                .push(RangeMapEntry::from_line(line));
        }
    }
    range_maps.push(current_range_map);
    let out1 = seeds
        .iter()
        .map(|seed| {
            range_maps
                .iter()
                .fold(*seed, |state, range_map| range_map.map_num(state))
        })
        .min();
    println!("{}", out1.unwrap());

    let seeds_ranges: Vec<_> = seeds
        .chunks_exact(2)
        .map(|chunk| (chunk[0]..(chunk[0] + chunk[1])))
        .collect();

    let min_seed = (0..)
        .find(|goal| {
            let seed = range_maps
                .iter()
                .rev()
                .fold(*goal, |state, range_map| range_map.map_num_rev(state));
            seeds_ranges.iter().any(|range| range.contains(&seed))
        })
        .unwrap();
    println!("{min_seed}");
}
