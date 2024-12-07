fn solve(lines: impl Iterator<Item = String>) -> (i64, i64) {
    let part1 = lines
        .filter_map(|line| {
            let mut parts = line.split(":");
            let result: i64 = parts.next().unwrap().parse().expect(format!("Failed to parse {}", line).as_str());
            let operands: Vec<i64> = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|operand| operand.parse().unwrap())
                .collect();

            for i in 0..(1 << (operands.len() - 1)) {
                let mut sum = operands[0];
                for j in 0..operands.len() - 1 {
                    if i & (1 << j) != 0 {
                        sum += operands[j + 1];
                    } else {
                        sum *= operands[j + 1];
                    }
                }

                if sum == result {
                    return Some(result);
                }
            }

            None
        })
        .sum();

    (part1, 0)
}

fn main() {
    let (part1, part2) = solve(util::stdin_lines());

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
