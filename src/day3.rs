use regex::Regex;
use util::stdin_lines;

fn main() {
    let mul_pattern = Regex::new(r#"mul\((\d+),(\d+)\)"#).unwrap();
    let total: i32 = stdin_lines()
        .map(|line| -> i32 {
            mul_pattern
                .captures_iter(&line)
                .map(|captures| {
                    let left: i32 = captures.get(1).unwrap().as_str().parse().unwrap();
                    let right: i32 = captures.get(2).unwrap().as_str().parse().unwrap();
                    left * right
                })
                .sum()
        })
        .sum();

    println!("{}", total);
}
