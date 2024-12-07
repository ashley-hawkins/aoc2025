use itertools::Itertools;

#[derive(Clone, Copy)]
enum Operator {
    Add,
    Mul,
    Concat,
}

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
            .unwrap_or_else(|_| panic!("Failed to parse {}", line));

        let operands: Vec<u64> = parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(|operand| operand.parse().unwrap())
            .collect();

        let starting_point = operands[0];

        for ops in std::iter::repeat([Operator::Add, Operator::Mul])
            .take(operands.len() - 1)
            .multi_cartesian_product()
        {
            let mut sum = starting_point;
            for (op, operand) in ops.iter().zip(operands.iter().skip(1)) {
                match op {
                    Operator::Mul => sum *= operand,
                    Operator::Add => sum += operand,
                    _ => unreachable!(),
                }
            }

            if sum == result {
                part1 += result;
                part2 += result;
                continue 'outer;
            }
        }

        for ops in std::iter::repeat([Operator::Add, Operator::Mul, Operator::Concat])
            .take(operands.len() - 1)
            .multi_cartesian_product()
        {
            let mut sum = starting_point;
            for (op, operand) in ops.iter().zip(operands.iter().skip(1)) {
                match op {
                    Operator::Mul => sum *= operand,
                    Operator::Add => sum += operand,
                    Operator::Concat => {
                        sum = concat(sum, *operand);
                    }
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
