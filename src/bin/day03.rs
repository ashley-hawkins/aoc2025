use util::stdin_lines;

fn max_joltage_with_digits(joltages: &[u8], digits: usize) -> u64 {
    let mut best_digits: Vec<usize> = Vec::with_capacity(digits);

    for i in 0..best_digits.capacity() {
        let start_idx = if let Some(&x) = best_digits.last() {
            x + 1
        } else {
            0
        };

        let max_joltage_index = joltages
            [start_idx..joltages.len() - best_digits.capacity() + i + 1]
            .iter()
            .enumerate()
            .rev()
            .max_by_key(|(_, &j)| j)
            .unwrap()
            .0
            + start_idx;
        best_digits.push(max_joltage_index);
    }

    let max_joltage = best_digits
        .iter()
        .fold(0u64, |acc, &idx| acc * 10 + joltages[idx] as u64);

    max_joltage
}

fn solve(lines: impl Iterator<Item = String>) -> (u64, u64) {
    let mut count: u64 = 0;
    let mut count2: u64 = 0;

    for line in lines {
        let joltages = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect::<Vec<_>>();

        let part1 = max_joltage_with_digits(&joltages, 2);
        count += part1;
        let part2 = max_joltage_with_digits(&joltages, 12);
        count2 += part2;
    }

    (count, count2)
}

fn main() {
    let (part1, part2) = solve(stdin_lines());

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
