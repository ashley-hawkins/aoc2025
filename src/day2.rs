use std::cmp::Ordering;

use util::stdin_lines;

fn classify_report(levels: &[i32]) -> bool {
    let mut direction: Option<Ordering> = None;
    for pair in levels.windows(2) {
        let left = pair[0];
        let right = pair[1];
        let this_direction = left.cmp(&right);

        if direction.is_none() {
            direction = Some(this_direction);
        } else if direction != Some(this_direction) {
            return false;
        }

        if !(1..=3).contains(&left.abs_diff(right)) {
            return false;
        }
    }

    true
}

fn classify_report_pt2(levels: &[i32]) -> (bool, bool) {
    let part1_succeeds = classify_report(levels);
    let part2_succeeds = part1_succeeds
        || (0..levels.len()).any(|i| {
            let mut line = levels.to_owned();
            line.remove(i);
            classify_report(&line)
        });

    (part1_succeeds, part2_succeeds)
}

fn solve(lines: impl Iterator<Item = String>) -> (i32, i32) {
    let mut part1 = 0;
    let mut part2 = 0;

    for report in lines {
        let levels: Vec<_> = report
            .split_ascii_whitespace()
            .map(|level| level.parse().unwrap())
            .collect();

        let (pt1, pt2) = classify_report_pt2(&levels);

        if pt1 {
            part1 += 1;
        }

        if pt2 {
            part2 += 1;
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
        let input = include_str!("../test_data/day2.txt");

        let (part1, part2) = solve(input.lines().map(str::to_owned));

        assert_eq!(part1, 2);
        assert_eq!(part2, 4);
    }
}
