use std::collections::HashMap;

use util::stdin_lines;

fn main() {
    let mut left = Vec::<i32>::new();
    let mut right = Vec::<i32>::new();
    let mut right_count = HashMap::<i32, usize>::new();

    for line in stdin_lines() {
        let mut parts = line.split_ascii_whitespace();
        left.push(parts.next().unwrap().parse().unwrap());
        let right_elem = parts.next().unwrap().parse().unwrap();
        right.push(right_elem);
        *right_count.entry(right_elem).or_insert(0) += 1;
    }

    left.sort();
    right.sort();

    let (part1, part2) = left
        .into_iter()
        .zip(right)
        .map(|(l, r)| {
            (
                l.abs_diff(r),
                l as usize * right_count.get(&l).cloned().unwrap_or(0),
            )
        })
        .fold((0, 0), |(acc_diff, acc_sum), (diff, sum)| {
            (acc_diff + diff, acc_sum + sum)
        });

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
