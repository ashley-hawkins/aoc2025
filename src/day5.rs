use std::collections::HashSet;

use util::stdin_lines;

fn main() {
    let mut lines = stdin_lines();

    // Rules
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

    // Updates
    let (part1, part2) = lines
        .map(|update| {
            let mut pages = update
                .split(",")
                .map(|element| element.parse().unwrap())
                .collect::<Vec<i32>>();

            let mut is_correct = true;
            for i in 0..pages.len() {
                for j in i + 1..pages.len() {
                    if rules.contains(&(pages[j], pages[i])) {
                        is_correct = false;
                        pages.sort_by(|a, b| {
                            if rules.contains(&(*b, *a)) {
                                std::cmp::Ordering::Greater
                            } else if rules.contains(&(*a, *b)) {
                                std::cmp::Ordering::Less
                            } else {
                                std::cmp::Ordering::Equal
                            }
                        });
                    }
                }
            }
            let mid = pages[pages.len() / 2];
            if is_correct {
                (mid, 0)
            } else {
                (0, mid)
            }
        })
        .fold((0, 0), |(acc1, acc2), (a, b)| (acc1 + a, acc2 + b));

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
