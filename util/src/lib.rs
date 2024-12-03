use std::io::{self, BufRead};

pub fn stdin_lines() -> impl Iterator<Item = String> {
    io::stdin().lock().lines().map(Result::unwrap)
}
