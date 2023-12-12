#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOak,
    FullHouse,
    FourOak,
    FiveOak,
}

impl Type {
    fn from_strengths_part1(strenghts: &[u8; 5]) -> Self {
        let mut counts = [0; 13];
        for s in strenghts {
            counts[*s as usize] += 1;
        }
        let max = counts.iter().max().unwrap();
        match max {
            5 => Self::FiveOak,
            4 => Self::FourOak,
            3 => {
                if counts.iter().any(|c| *c == 2) {
                    Self::FullHouse
                } else {
                    Self::ThreeOak
                }
            }
            2 => {
                if counts.iter().filter(|c| **c == 2).count() == 2 {
                    Self::TwoPair
                } else {
                    Self::OnePair
                }
            }
            1 => Self::HighCard,
            _ => panic!("{max} how?"),
        }
    }

    fn from_strengths_part2(strenghts: &[u8; 5]) -> Self {
        let mut counts = [0u8; 13];
        for s in strenghts {
            counts[*s as usize] += 1;
        }
        // the joker is special
        let joker_count = counts[0];
        let non_joker_counts = &counts[1..];
        let max = non_joker_counts.iter().max().unwrap();
        // joker plus random card
        match joker_count {
            0 => Self::HighCard,
            1 => Self::OnePair,
            2 => Self::ThreeOak,
            3 => Self::FourOak,
            _ => Self::FiveOak,
        }
        .max(match max + joker_count {
            5 => Self::FiveOak,
            4 => Self::FourOak,
            3 => {
                // either 2:2 (had joker) or 3:2 (no joker)
                if non_joker_counts
                    .iter()
                    .filter(|c| **c >= 2)
                    .cloned()
                    .sum::<u8>()
                    >= 4
                {
                    Self::FullHouse
                } else {
                    Self::ThreeOak
                }
            }
            2 => {
                // either 2:1:1 (had joker) or 2:2:1 (no joker)
                let two_count = non_joker_counts.iter().filter(|c| **c == 2).count();
                if two_count == 2 || (joker_count == 1 && two_count == 1) {
                    Self::TwoPair
                } else {
                    Self::OnePair
                }
            }
            1 => Self::HighCard,
            _ => panic!("{max} how?"),
        })
    }
}

fn strength_from_char_part1(c: u8) -> u8 {
    match c {
        b'2' => 0,
        b'3' => 1,
        b'4' => 2,
        b'5' => 3,
        b'6' => 4,
        b'7' => 5,
        b'8' => 6,
        b'9' => 7,
        b'T' => 8,
        b'J' => 9,
        b'Q' => 10,
        b'K' => 11,
        b'A' => 12,
        _ => panic!("invalid char {c}"),
    }
}

fn strength_from_char_part2(c: u8) -> u8 {
    match c {
        b'J' => 0,
        b'2' => 1,
        b'3' => 2,
        b'4' => 3,
        b'5' => 4,
        b'6' => 5,
        b'7' => 6,
        b'8' => 7,
        b'9' => 8,
        b'T' => 9,
        b'Q' => 10,
        b'K' => 11,
        b'A' => 12,
        _ => panic!("invalid char {c}"),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    typ: Type,
    strengths: [u8; 5],
}

pub fn main() {
    let input = include_str!("../input/07.txt");
    //     let input = "32T3K 765
    // T55J5 684
    // KK677 28
    // KTJJT 220
    // QQQJA 483";
    let mut hands_bids: Vec<_> = input
        .lines()
        .map(|line| {
            let (cards, bid_s) = line.split_once(" ").unwrap();
            let bid: u16 = bid_s.parse().unwrap();
            // why is going from slice to fixed array so dumb
            let strengths: [u8; 5] = <&[u8] as TryInto<&[u8; 5]>>::try_into(cards.as_bytes())
                .unwrap()
                .map(|c| strength_from_char_part1(c));
            let typ = Type::from_strengths_part1(&strengths);
            (Hand { typ, strengths }, bid)
        })
        .collect();
    hands_bids.sort_by(|k0, k1| k0.0.cmp(&k1.0));
    let out1: usize = hands_bids
        .iter()
        .enumerate()
        .map(|(rank, (_hand, bid))| {
            // println!("{rank}: {bid}");
            (rank + 1) * (*bid as usize)
        })
        .sum();
    println!("{out1}");
    let mut hands_bids2: Vec<_> = input
        .lines()
        .map(|line| {
            let (cards, bid_s) = line.split_once(" ").unwrap();
            let bid: u16 = bid_s.parse().unwrap();
            // why is going from slice to fixed array so dumb
            let strengths: [u8; 5] = <&[u8] as TryInto<&[u8; 5]>>::try_into(cards.as_bytes())
                .unwrap()
                .map(|c| strength_from_char_part2(c));
            let typ = Type::from_strengths_part2(&strengths);
            (Hand { typ, strengths }, bid)
        })
        .collect();
    hands_bids2.sort_by(|k0, k1| k0.0.cmp(&k1.0));
    let out2: usize = hands_bids2
        .iter()
        .enumerate()
        .map(|(rank, (_hand, bid))| {
            // println!("{rank}: {bid}");
            // if !hand.strengths.iter().any((|c| *c == 0)) {
            //     println!("{hand:?}");
            // }
            (rank + 1) * (*bid as usize)
        })
        .sum();
    println!("{out2}");
}
