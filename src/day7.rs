fn concat(a: u64, b: u64) -> u64 {
    a * 10u64.pow(b.checked_ilog10().unwrap_or(0) + 1) + b
}

fn solve(lines: impl Iterator<Item = String>) -> (u64, u64) {
    let mut part1 = 0;
    let mut part2 = 0;

    'outer: for line in lines {
        let mut parts = line.split(":");
        let result: u64 = parts
            .next()
            .unwrap()
            .parse()
            .expect(format!("Failed to parse {}", line).as_str());
        let operands: Vec<u64> = parts
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
                part1 += result;
                part2 += result;
                continue 'outer;
            }
        }

        for i in 0..(3i64.pow((operands.len() - 1) as u32)) {
            let mut sum = operands[0];
            for j in 0..operands.len() - 1 {
                let op = (i / 3i64.pow(j as u32)) % 3;
                match op {
                    0 => sum *= operands[j + 1],
                    1 => sum += operands[j + 1],
                    2 => {
                        sum = concat(sum, operands[j + 1]);
                    }
                    _ => unreachable!(),
                }
            }

            if sum == result {
                part2 += result;
                continue 'outer;
            }
        }
    }

    (part1, part2)
}

fn main() {
    let (part1, part2) = solve(util::stdin_lines());

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concat() {
        for i in 0..1000 {
            for j in 0..1000 {
                assert_eq!(
                    concat(i, j),
                    (i.to_string() + &j.to_string()).parse().unwrap()
                );
            }
        }
    }
}
