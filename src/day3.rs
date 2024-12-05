use regex::Regex;

use util::stdin_lines;

fn solve(lines: impl Iterator<Item = String>) -> (i32, i32) {
    let instruction_pattern = Regex::new(r#"do\(\)|don't\(\)|mul\((\d+),(\d+)\)"#).unwrap();

    let mut part1: i32 = 0;
    let mut part2: i32 = 0;

    let mut active = true;
    for line in lines {
        for captures in instruction_pattern.captures_iter(&line) {
            let main_group = captures.get(0).unwrap();

            match main_group.as_str() {
                "do()" => {
                    active = true;
                }
                "don't()" => {
                    active = false;
                }
                _ => {
                    let left: i32 = captures.get(1).unwrap().as_str().parse().unwrap();
                    let right: i32 = captures.get(2).unwrap().as_str().parse().unwrap();
                    let res = left * right;
                    part1 += res;
                    if active {
                        part2 += res;
                    }
                }
            }
        }
    }

    (part1, part2)
}

fn main() {
    let (part1, part2) = solve(stdin_lines());

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = include_str!("../test_data/day3.txt")
            .lines()
            .map(str::to_owned);

        let (part1, part2) = solve(input);

        assert_eq!(part1, 161);
        assert_eq!(part2, 48);
    }
}
