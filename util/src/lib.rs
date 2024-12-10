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

pub fn lines_to_grid(lines: impl Iterator<Item = String>) -> ndarray::Array2<u8> {
    let lines = lines.collect::<Vec<_>>();

    let width = lines[0].len();
    let height = lines.len();

    let grid = lines
        .into_iter()
        .flat_map(String::into_bytes)
        .collect::<Vec<_>>();

    let grid = ndarray::Array2::from_shape_vec((height, width), grid).unwrap();

    grid
}
