use std::collections::HashSet;

use util::stdin_lines;

fn solve(mut lines: impl Iterator<Item = String>) -> (i32, i32) {
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

    let is_update_correct = |pages: &[i32]| -> bool {
        for i in 0..pages.len() {
            for j in i + 1..pages.len() {
                if rules.contains(&(pages[j], pages[i])) {
                    return false;
                }
            }
        }
        true
    };

    let mut part1 = 0;
    let mut part2 = 0;
    for update in lines {
        let mut pages = update
            .split(",")
            .map(|element| element.parse().unwrap())
            .collect::<Vec<i32>>();

        let is_correct = is_update_correct(&pages);

        if !is_correct {
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

        let mid = pages[pages.len() / 2];
        if is_correct {
            part1 += mid;
        } else {
            part2 += mid;
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
        let input = include_str!("../test_data/day05.txt");

        let (part1, part2) = solve(input.lines().map(str::to_owned));

        assert_eq!(part1, 143);
        assert_eq!(part2, 123);
    }
}
