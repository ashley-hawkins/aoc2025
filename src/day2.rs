use std::{cmp::Ordering, io};

fn classify_report(report: &Vec<&str>) -> bool {
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

fn classify_report_pt1(report: &str) -> bool {
    classify_report(&report.split_ascii_whitespace().collect())
}

fn classify_report_pt2(report: &str) -> bool {
    let line = report.split_ascii_whitespace().collect();

    classify_report(&line)
        || (0..line.len()).any(|i| {
            let mut line = line.clone();
            line.remove(i);
            classify_report(&line)
        })
}

fn main() {
    let mut report: String = String::new();

    let mut count_1 = 0;
    let mut count_2 = 0;
    while matches!(io::stdin().read_line(&mut report), Ok(n) if n > 0) {
        if classify_report_pt1(&report) {
            count_1 += 1;
        }
        if classify_report_pt2(&report) {
            count_2 += 1;
        }
        report.clear();
    }

    println!("Part 1: {} / Part 2: {}", count_1, count_2);
}
