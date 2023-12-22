pub fn main() {
    let input = include_str!("../input/09.txt");

    let out1: isize = input
        .lines()
        .map(|line| calculate_result(line, false))
        .sum();
    println!("{out1}");

    let out2: isize = input.lines().map(|line| calculate_result(line, true)).sum();
    println!("{out2}");
}

fn calculate_result(line: &str, use_first: bool) -> isize {
    let mut nums: Vec<isize> = line.split(' ').map(|s| s.parse().unwrap()).collect();
    let mut edge_nums = Vec::new();
    loop {
        if use_first {
            edge_nums.push(nums.first().cloned().unwrap());
        } else {
            edge_nums.push(nums.last().cloned().unwrap());
        }
        let new_nums: Vec<isize> = nums.windows(2).map(|c| c[1] - c[0]).collect();
        if new_nums.iter().all(|n| *n == 0) {
            break;
        }
        nums = new_nums;
    }
    // println!("{:?}", edge_nums);
    if use_first {
        edge_nums.iter().rev().fold(0, |acc, num| *num - acc)
    } else {
        edge_nums.iter().rev().fold(0, |acc, num| acc + *num)
    }
}
