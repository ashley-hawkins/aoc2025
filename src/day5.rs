use std::collections::HashSet;

use util::stdin_lines;

fn main() {
    let mut lines = stdin_lines();

    let rules = {
        let rules_iter = lines.by_ref().take_while(|line| !line.is_empty());
        rules_iter
            .map(|line| {
                let mut parts = line.split("|");
                let part1: i32 = parts.next().unwrap().parse().unwrap();
                let part2: i32 = parts.next().unwrap().parse().unwrap();

                (part1, part2)
            })
            .collect::<HashSet<_>>()
    };

    let results: i32 = lines
        .by_ref()
        .map(|line| {
            let mut parts = line
                .split(",")
                .map(|element| element.parse().unwrap())
                .collect::<Vec<i32>>();

            let mut is_correct = true;
            for i in 0..parts.len() {
                for j in i + 1..parts.len() {
                    if rules.contains(&(parts[j], parts[i])) {
                        is_correct = false;
                        parts.swap(i, j);
                    }
                }
            }
            if is_correct {
                return 0;
            }
            parts[parts.len() / 2]
        })
        .sum();

    println!("Result: {}", results);
}
