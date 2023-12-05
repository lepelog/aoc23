pub fn main() {
    let input = include_str!("../input/01.txt");
    let out1: usize = input
        .lines()
        .map(|line| {
            let idx1 = line.find(|c| matches!(c, '0'..='9')).unwrap();
            let idx2 = line.rfind(|c| matches!(c, '0'..='9')).unwrap();
            let num1 = line.bytes().nth(idx1).unwrap() - b'0';
            let num2 = line.bytes().nth(idx2).unwrap() - b'0';
            (num1 * 10 + num2) as usize
        })
        .sum();
    println!("{out1}");

    let numbers: [(&[u8], usize); 18] = [
        (b"1", 1),
        (b"2", 2),
        (b"3", 3),
        (b"4", 4),
        (b"5", 5),
        (b"6", 6),
        (b"7", 7),
        (b"8", 8),
        (b"9", 9),
        (b"one", 1),
        (b"two", 2),
        (b"three", 3),
        (b"four", 4),
        (b"five", 5),
        (b"six", 6),
        (b"seven", 7),
        (b"eight", 8),
        (b"nine", 9),
    ];

    let out2: usize = input
        .as_bytes()
        .split(|b| *b == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let first_num = (0..line.len())
                .map(|i| &line[i..])
                .find_map(|subline| {
                    for (pat, value) in numbers {
                        if subline.starts_with(pat) {
                            return Some(value);
                        }
                    }
                    None
                })
                .unwrap();
            let last_num = (0..line.len())
                .rev()
                .map(|i| &line[i..])
                .find_map(|subline| {
                    for (pat, value) in numbers {
                        if subline.starts_with(pat) {
                            return Some(value);
                        }
                    }
                    None
                })
                .unwrap();
            (first_num * 10 + last_num) as usize
        })
        .sum();
    println!("{out2}");
}
