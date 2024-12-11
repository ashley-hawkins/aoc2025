use std::{collections::HashMap, num::NonZero};

use rayon::iter::{IntoParallelIterator, IntoParallelRefMutIterator, ParallelIterator};
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

/*
fn transform_hashmap(input: &mut HashMap<i64, u32>) {
    input.iter
}

fn solve(mut lines: impl Iterator<Item = String>) -> (i64, i64) {
    let mut the_numbers = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .fold(HashMap::new(), |mut acc, n| {
            *acc.entry(n).or_insert(0) += 1u32;
            acc
        });

    for i in 0..75 {
        transform_vec(&mut the_numbers);
        println!("{i}");
    }

    println!("{:?}", the_numbers.len());

    (0, 0)
}

*/

fn transform_vec<'a>(input: HashMap<i64, u64>) -> HashMap<i64, u64> {
    let mapper = |(n, count)| {
        let (l, r) = transform(n);
        if let Some(l) = l {
            vec![(l, count), (r, count)]
        } else {
            vec![(r, count)]
        }
    };
    input
        .into_par_iter()
        .flat_map(mapper)
        .flat_map(mapper)
        .flat_map(mapper)
        .flat_map(mapper)
        .flat_map(mapper)
        .fold(
            || HashMap::new(),
            |mut acc, (n, count)| {
                *acc.entry(n).or_insert(0) += count;
                acc
            },
        )
        .reduce(
            || HashMap::new(),
            |mut acc, rhs| {
                for (n, count) in rhs {
                    *acc.entry(n).or_insert(0) += count;
                }
                acc
            },
        )
}

fn solve(mut lines: impl Iterator<Item = String>) -> (u64, u64) {
    let mut the_numbers = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .fold(HashMap::new(), |mut acc, n| {
            *acc.entry(n).or_insert(0) += 1u64;
            acc
        });

    let mut res = the_numbers;

    for i in 0..5 {
        res = transform_vec(res);
    }

    let part1 = res.iter().fold(0, |acc, (_, &v)| acc + v);

    // let mut count_the_numbers = the_numbers.iter().fold(HashMap::new(), |mut acc, n| {
    //     *acc.entry(n).or_insert(0) += 1u32;
    //     acc
    // });

    for i in 0..10 {
        res = transform_vec(res);
    }

    let part2 = res.iter().fold(0, |acc, (_, &v)| acc + v);
    // let max_duplicate = count_the_numbers.iter().max_by_key(|(_, &v)| v).unwrap().0;
    // println!("Max duplicate: {:?}", max_duplicate);

    // println!("{:?}", the_numbers.len());

    println!("{:?}", res.len());

    (part1, part2)
}

fn main() {
    let (part1, part2) = solve(stdin_lines());

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
