use regex::Regex;
use util::stdin_lines;

fn main() {
    let instruction_pattern = Regex::new(r#"do\(\)|don't\(\)|mul\((\d+),(\d+)\)"#).unwrap();

    let mut active = true;
    let (part1, part2) = stdin_lines()
        .map(|line| -> (i32, i32) {
            instruction_pattern
                .captures_iter(&line)
                .filter_map(|captures| {
                    let main_group = captures.get(0).unwrap();

                    match main_group.as_str() {
                        "do()" => {
                            active = true;
                            None
                        }
                        "don't()" => {
                            active = false;
                            None
                        }
                        _ => {
                            let left: i32 = captures.get(1).unwrap().as_str().parse().unwrap();
                            let right: i32 = captures.get(2).unwrap().as_str().parse().unwrap();
                            let res = left * right;
                            if !active {
                                Some((res, 0))
                            } else {
                                Some((res, res))
                            }
                        }
                    }
                })
                .fold((0, 0), |(acc1, acc2), (val1, val2)| {
                    (acc1 + val1, acc2 + val2)
                })
        })
        .fold((0, 0), |(acc1, acc2), (val1, val2)| {
            (acc1 + val1, acc2 + val2)
        });

    println!("Part 1: {part1} / Part 2: {part2}");
}
