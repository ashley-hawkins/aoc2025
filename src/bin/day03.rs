use util::stdin_lines;

fn solve(lines: impl Iterator<Item = String>) -> (u64, u64) {
    let mut dial: i32 = 50;
    let mut count: u64 = 0;

    for line in lines {
        // guaranteed to be ascii
        let chars = line.as_bytes();

        let mut max_joltage_index: usize = 0;
        for i in 0..chars.len() - 1 {
            let joltage = chars[i] - b'0';

            if joltage > chars[max_joltage_index] - b'0' {
                max_joltage_index = i;
            }
        }

        let max_joltage = ((chars[max_joltage_index] - b'0') * 10)
            + chars[max_joltage_index + 1..].iter().map(|x| x - b'0').max().unwrap();
        count += max_joltage as u64;
    }

    (count, 0)
}

fn main() {
    let (part1, part2) = solve(stdin_lines());

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
