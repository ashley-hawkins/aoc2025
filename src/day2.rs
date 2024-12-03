use std::{cmp::Ordering, io};

fn classify_report(report: &str) -> bool {
    let mut direction: Option<Ordering> = None;
    for pair in report
        .split_ascii_whitespace()
        .collect::<Vec<_>>()
        .windows(2)
    {
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

fn main() {
    let mut report: String = String::new();

    let mut count = 0;
    while matches!(io::stdin().read_line(&mut report), Ok(n) if n > 0) {
        if classify_report(&report) {
            count += 1;
        }
        report.clear();
    }

    println!("Count: {}", count);
}
