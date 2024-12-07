fn solve(lines: impl Iterator<Item = String>) -> (i64, i64) {
    let mut part1 = 0;
    let mut part2 = 0;

    'outer: for line in lines {
        let mut parts = line.split(":");
        let result: i64 = parts
            .next()
            .unwrap()
            .parse()
            .expect(format!("Failed to parse {}", line).as_str());
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
                        sum = (sum.to_string() + &operands[j + 1].to_string())
                            .parse()
                            .unwrap()
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
