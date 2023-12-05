use nom::{bytes::complete::tag, character::complete::digit1, combinator::map_res, IResult};

pub fn main() {
    let input = include_str!("../input/02.txt");
    let out1: usize = input
        .lines()
        .map(|line| parse_line(line).unwrap())
        .filter_map(|(_, (game_num, grabs))| {
            grabs.iter().all(SingleGrab::is_valid).then_some(game_num)
        })
        .sum();
    println!("{}", out1);
    let out2: usize = input
        .lines()
        .map(|line| parse_line(line).unwrap())
        .map(|(_, (_, grabs))| calculate_power(&grabs))
        .sum();
    println!("{}", out2);
}

#[derive(Debug)]
struct SingleGrab {
    red: u8,
    green: u8,
    blue: u8,
}

impl SingleGrab {
    fn is_valid(&self) -> bool {
        return self.red <= 12 && self.green <= 13 && self.blue <= 14;
    }
}

fn calculate_power(grabs: &[SingleGrab]) -> usize {
    let red = grabs.iter().map(|g| g.red).max().unwrap_or(0);
    let green = grabs.iter().map(|g| g.green).max().unwrap_or(0);
    let blue = grabs.iter().map(|g| g.blue).max().unwrap_or(0);
    red as usize * green as usize * blue as usize
}

fn parse_line(l: &str) -> IResult<&str, (usize, Vec<SingleGrab>)> {
    let (l, _) = tag("Game ")(l)?;
    let (l, game_num) = map_res(digit1, str::parse)(l)?;
    let (l, _) = tag(": ")(l)?;
    let grabs = l
        .split("; ")
        .map(|item| {
            let mut grab = SingleGrab {
                red: 0,
                green: 0,
                blue: 0,
            };
            for part in item.split(", ") {
                let (num, color) = part.split_once(" ").unwrap();
                let parsed_num: u8 = num.parse().unwrap();
                match color {
                    "red" => grab.red = parsed_num,
                    "green" => grab.green = parsed_num,
                    "blue" => grab.blue = parsed_num,
                    _ => unreachable!(),
                }
            }
            grab
        })
        .collect();
    Ok((l, (game_num, grabs)))
}
