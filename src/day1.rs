use std::collections::HashMap;

use util::stdin_lines;

fn main() {
    let mut left = Vec::<i32>::new();
    let mut right = Vec::<i32>::new();
    let mut right_count = HashMap::<i32, usize>::new();

    for line in stdin_lines() {
        let mut parts = line.split_ascii_whitespace();
        let left_elem = parts.next().unwrap().parse().unwrap();
        let right_elem = parts.next().unwrap().parse().unwrap();

        left.push(left_elem);
        right.push(right_elem);
        *right_count.entry(right_elem).or_insert(0) += 1;
    }

    left.sort();
    right.sort();

    let mut part1 = 0;
    let mut part2 = 0;
    for (left, right) in left.into_iter().zip(right) {
        part1 += left.abs_diff(right);
        part2 += left as usize * right_count.get(&left).cloned().unwrap_or(0)
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
