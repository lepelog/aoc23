use std::collections::HashSet;

pub fn main() {
    let input = include_str!("../input/04.txt");
    //     let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    // Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    // Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    // Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    // Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    // Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    // "#;
    let all_scores: Vec<_> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (win_raw, my_raw) = line[10..].split_once('|').unwrap();
            let win: HashSet<u8> = win_raw
                .split_ascii_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();
            let my: HashSet<u8> = my_raw
                .split_ascii_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();
            let same_count = win.intersection(&my).count();
            same_count
        })
        .collect();
    let out1: usize = all_scores
        .iter()
        .map(|same_count| same_count.checked_sub(1).map_or(0, |val| 1 << val))
        .sum();
    println!("{out1}");

    println!("{:?}", all_scores);

    let mut card_counts = vec![1usize; all_scores.len()];
    for (i, score) in all_scores.iter().enumerate() {
        let current_card_count = card_counts[i];
        for card_count in card_counts
            .get_mut((i + 1)..(i + 1 + *score).min(all_scores.len()))
            .unwrap_or_default()
            .iter_mut()
        {
            *card_count += current_card_count;
        }
    }
    let out2: usize = card_counts.iter().sum();
    println!("{:?}", card_counts);
    println!("{out2}");
}
