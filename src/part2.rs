use std::{collections::HashMap, io};

fn main() {
    let mut left = HashMap::<i32, usize>::new();
    let mut right = HashMap::<i32, usize>::new();
    let mut line: String = String::new();
    while match io::stdin().read_line(&mut line) {
        Ok(0) => false,
        Ok(_) => true,
        Err(_) => false,
    } {
        let mut parts = line.split_ascii_whitespace();
        *left
            .entry(parts.next().unwrap().parse().unwrap())
            .or_insert(0) += 1;
        *right
            .entry(parts.next().unwrap().parse().unwrap())
            .or_insert(0) += 1;
        line.clear();
    }

    let result: usize = left
        .into_iter()
        .map(|(k, count)| k as usize * right.get(&k).cloned().unwrap_or(0) * count)
        .sum();
    println!("{}", result);
}
