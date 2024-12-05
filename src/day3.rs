use regex::Regex;
use util::stdin_lines;

fn main() {
    let instruction_pattern = Regex::new(r#"do\(\)|don't\(\)|mul\((\d+),(\d+)\)"#).unwrap();

    let mut active = true;

    let mut part1: i32 = 0;
    let mut part2: i32 = 0;

    for line in stdin_lines() {
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

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
