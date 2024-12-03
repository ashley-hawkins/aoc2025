use std::{cmp::Ordering, io};

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
    let mut report: String = String::new();

    let mut count_1 = 0;
    let mut count_2 = 0;
    while matches!(io::stdin().read_line(&mut report), Ok(n) if n > 0) {
        let report_vec: Vec<_> = report.split_ascii_whitespace().collect();
        let (pt1, pt2) = classify_report_pt2(&report_vec);

        if pt1 {
            count_1 += 1;
        }

        if pt2 {
            count_2 += 1;
        }

        report.clear();
    }

    println!("Part 1: {} / Part 2: {}", count_1, count_2);
}
