use std::collections::HashMap;

use num_integer::Integer;

pub fn main() {
    let input = include_str!("../input/08.txt");
    //     let input = "LLR

    // AAA = (BBB, BBB)
    // BBB = (AAA, ZZZ)
    // ZZZ = (ZZZ, ZZZ)";
    let mut lines = input.lines();
    // left is false, right is true
    let directions = lines.next().unwrap().bytes().map(|c| c == b'R').cycle();
    let _ = lines.next();
    let dir_map: HashMap<String, (String, String)> = lines
        .map(|line| {
            let key = line[0..3].to_string();
            let left = line[7..10].to_string();
            let right = line[12..15].to_string();
            (key, (left, right))
        })
        .collect();
    let out1 = get_follow_distance(directions.clone(), &dir_map, "AAA", |key| key == "ZZZ");
    println!("{out1}");
    let out2 = dir_map
        .keys()
        .filter(|key| key.ends_with('A'))
        .map(|key| get_follow_distance(directions.clone(), &dir_map, key, |k| k.ends_with('Z')))
        .fold(1usize, |state, item| item.lcm(&state));
    println!("{out2}");
}

fn get_follow_distance(
    mut directions: impl Iterator<Item = bool>,
    dir_map: &HashMap<String, (String, String)>,
    start: &str,
    is_at_end: impl Fn(&str) -> bool,
) -> usize {
    let Err(final_count) = directions.try_fold((start, 1usize), |(key, count), go_right| {
        let dirs = &dir_map[key];
        let new_key = if go_right {
            dirs.1.as_str()
        } else {
            dirs.0.as_str()
        };
        if is_at_end(new_key) {
            return Err(count);
        } else {
            return Ok((new_key, count + 1));
        }
    }) else {
        unreachable!();
    };
    final_count
}
