use std::cmp::Ordering;

use util::stdin_lines;

fn classify_report(report: &[&str]) -> bool {
    let mut direction: Option<Ordering> = None;
    for pair in report.windows(2) {
        let left = pair[0].parse::<i32>().unwrap();
        let right = pair[1].parse::<i32>().unwrap();
        let this_direction = left.cmp(&right);

        if direction.is_none() {
            if direction == Some(Ordering::Equal) {
                return false;
            }
            direction = Some(this_direction);
        }

        if direction != Some(this_direction) {
            return false;
        }

        if !(1..=3).contains(&left.abs_diff(right)) {
            return false;
        }
    }

    true
}

fn classify_report_pt2(report: &[&str]) -> (bool, bool) {
    let part1_succeeds = classify_report(report);
    let part2_succeeds = part1_succeeds
        || (0..report.len()).any(|i| {
            let mut line = report.to_owned();
            line.remove(i);
            classify_report(&line)
        });

    (part1_succeeds, part2_succeeds)
}

fn main() {
    let mut part1 = 0;
    let mut part2 = 0;
    for report in stdin_lines() {
        let report_vec: Vec<_> = report.split_ascii_whitespace().collect();
        let (pt1, pt2) = classify_report_pt2(&report_vec);

        if pt1 {
            part1 += 1;
        }

        if pt2 {
            part2 += 1;
        }
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
