use std::{
    io::{self, BufRead, BufReader},
    path::Path,
};

pub fn stdin_lines() -> impl Iterator<Item = String> {
    stream_lines(io::stdin().lock())
}

pub fn file_lines(p: impl AsRef<Path>) -> impl Iterator<Item = String> {
    stream_lines(BufReader::new(std::fs::File::open(p).unwrap()))
}

pub fn stream_lines(stream: impl BufRead) -> impl Iterator<Item = String> {
    stream.lines().map(Result::unwrap)
}
