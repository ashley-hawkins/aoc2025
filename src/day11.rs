use std::collections::HashMap;

use util::stdin_lines;

fn transform(input: i64) -> (Option<i64>, i64) {
    const RADIX: i64 = 10;
    if input == 0 {
        return (None, 1);
    }
    let digits = input.ilog(RADIX) + 1;
    if digits % 2 == 0 {
        let half_digits = 10i64.pow(digits / 2);
        let l = input / half_digits;
        let r = input % half_digits;
        return (Some(l), r);
    }

    (None, input * 2024)
}

fn transform_map(input: HashMap<i64, u64>) -> HashMap<i64, u64> {
    let mapper = |(n, count)| {
        let (l, r) = transform(n);
        if let Some(l) = l {
            vec![(l, count), (r, count)]
        } else {
            vec![(r, count)]
        }
    };
    input
        .into_iter()
        .flat_map(mapper)
        .fold(HashMap::new(), |mut acc, (n, count)| {
            *acc.entry(n).or_insert(0) += count;
            acc
        })
}

fn solve(mut lines: impl Iterator<Item = String>) -> (u64, u64) {
    let starting_numbers = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .fold(HashMap::new(), |mut acc, n| {
            *acc.entry(n).or_insert(0) += 1u64;
            acc
        });

    let mut res = starting_numbers;

    for _ in 0..25 {
        res = transform_map(res);
    }

    let part1 = res.iter().fold(0, |acc, (_, &v)| acc + v);

    for _ in 0..50 {
        res = transform_map(res);
    }

    let part2 = res.iter().fold(0, |acc, (_, &v)| acc + v);

    (part1, part2)
}

fn main() {
    let (part1, part2) = {
        let _stopwatch = util::ScopedStopwatch::new(|duration| {
            eprintln!("Time: {:?}", duration);
        });
        solve(stdin_lines())
    };

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
