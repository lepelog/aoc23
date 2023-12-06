use aoc23::{chr_to_num, AocGrid};

enum LineState {
    NumSearch,
    NumFound { has_sumbol: bool, value: usize },
}

fn check_any_neightbor_symbol(grid: &AocGrid, x: usize, y: usize) -> bool {
    check_any_neightbor_matches(grid, x, y, &|chr| !matches!(chr, b'0'..=b'9' | b'.'))
}

fn check_any_neightbor_matches(
    grid: &AocGrid,
    x: usize,
    y: usize,
    f: &impl Fn(u8) -> bool,
) -> bool {
    for xd in [-1, 0, 1] {
        for yd in [-1, 0, 1] {
            if check_matches(grid, x.checked_add_signed(xd), y.checked_add_signed(yd), f) {
                return true;
            }
        }
    }
    false
}

fn check_matches(
    grid: &AocGrid,
    x: Option<usize>,
    y: Option<usize>,
    f: &impl Fn(u8) -> bool,
) -> bool {
    let (Some(x), Some(y)) = (x, y) else {
        return false;
    };
    let Some(chr) = grid.get(x, y) else {
        return false;
    };
    f(chr)
}

fn solve_part1(grid: &AocGrid) {
    let mut sum_part_numbers = 0;
    for y in 0..grid.height {
        let mut state = LineState::NumSearch;
        for x in 0..grid.width {
            let chr = grid.get(x, y).unwrap();
            let maybe_num = matches!(chr, b'0'..=b'9').then(|| (chr - b'0') as usize);
            match (&state, maybe_num) {
                (LineState::NumSearch, Some(num)) => {
                    state = LineState::NumFound {
                        has_sumbol: check_any_neightbor_symbol(&grid, x, y),
                        value: num,
                    };
                }
                (LineState::NumSearch, None) => (),
                (LineState::NumFound { has_sumbol, value }, Some(num)) => {
                    state = LineState::NumFound {
                        has_sumbol: *has_sumbol || check_any_neightbor_symbol(&grid, x, y),
                        value: value * 10 + num,
                    }
                }
                (LineState::NumFound { has_sumbol, value }, None) => {
                    if *has_sumbol {
                        sum_part_numbers += *value;
                    }
                    state = LineState::NumSearch;
                }
            }
        }
        match &state {
            LineState::NumFound { has_sumbol, value } => {
                if *has_sumbol {
                    sum_part_numbers += *value;
                }
            }
            _ => (),
        }
    }
    println!("{sum_part_numbers}");
}

fn build_num_both(grid: &AocGrid, mut x: usize, y: usize) -> usize {
    if !matches!(grid.get(x, y), Some(b'0'..=b'9')) {
        return 0;
    }
    // go left as long as there are numbers
    // if x > 0 {
    while x > 0 && matches!(grid.get(x - 1, y), Some(b'0'..=b'9')) {
        x -= 1;
    }
    // }
    build_num_right(grid, x, y)
}

fn build_num_right(grid: &AocGrid, mut x: usize, y: usize) -> usize {
    let mut sum = 0;
    while let Some(num) = grid.get(x, y).and_then(chr_to_num) {
        sum = sum * 10 + num as usize;
        x += 1;
    }
    sum
}

fn build_num_left(grid: &AocGrid, mut x: usize, y: usize) -> usize {
    let mut sum = 0;
    let mut digit_weight = 1;
    while let Some(num) = grid.get(x, y).and_then(chr_to_num) {
        sum = sum + num as usize * digit_weight;
        digit_weight *= 10;
        if x > 0 {
            x -= 1;
        } else {
            break;
        }
    }
    sum
}

fn solve_part2(grid: &AocGrid) {
    let mut sum_gear_numbers = 0;
    for y in 0..grid.height {
        for x in 0..grid.width {
            if grid.get(x, y).unwrap() == b'*' {
                // TODO: find all spots around that are numbers
                // TODO: function to search number to left/right

                let mut found_nums = vec![];
                // left:
                let left = build_num_left(grid, x.wrapping_sub(1), y);
                if left > 0 {
                    found_nums.push(left);
                }
                // right:
                let right = build_num_right(grid, x + 1, y);
                if right > 0 {
                    found_nums.push(right);
                }
                // bottom, check middle first
                // either the middle has a number, or the sides needs to be checked
                let bottom = build_num_both(grid, x, y + 1);
                if bottom > 0 {
                    found_nums.push(bottom);
                    // the bottom corners don't need to be checked
                } else {
                    // check corners
                    let bottom_left = build_num_left(grid, x.wrapping_sub(1), y + 1);
                    if bottom_left > 0 {
                        found_nums.push(bottom_left);
                    }
                    let bottom_right = build_num_right(grid, x + 1, y + 1);
                    if bottom_right > 0 {
                        found_nums.push(bottom_right);
                    }
                }
                // top, check middle first
                // either the middle has a number, or the sides needs to be checked
                let top = build_num_both(grid, x, y.wrapping_sub(1));
                if top > 0 {
                    found_nums.push(top);
                    // the bottom corners don't need to be checked
                } else {
                    // check corners
                    let top_left = build_num_left(grid, x.wrapping_sub(1), y.wrapping_sub(1));
                    if top_left > 0 {
                        found_nums.push(top_left);
                    }
                    let top_right = build_num_right(grid, x + 1, y.wrapping_sub(1));
                    if top_right > 0 {
                        found_nums.push(top_right);
                    }
                }
                if found_nums.len() == 2 {
                    sum_gear_numbers += found_nums[0] * found_nums[1];
                }
            }
        }
    }
    println!("{sum_gear_numbers}")
}

pub fn main() {
    let grid = AocGrid::from_input(include_bytes!("../input/03.txt"));
    println!("{}:{}", grid.width, grid.height);
    solve_part1(&grid);
    solve_part2(&grid);
}
