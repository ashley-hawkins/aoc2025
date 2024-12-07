use std::collections::HashSet;

use ndarray::{Array2, ArrayView2};

enum Result {
    Exited,
    InfiniteLoop,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up = b'U',
    Down = b'D',
    Left = b'L',
    Right = b'R',
}

impl Direction {
    fn from_byte(byte: u8) -> Self {
        match byte {
            b'U' => Self::Up,
            b'D' => Self::Down,
            b'L' => Self::Left,
            b'R' => Self::Right,
            _ => panic!("Invalid direction"),
        }
    }

    fn to_byte(self) -> u8 {
        self as u8
    }

    fn rotate(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    fn to_vector(self) -> (isize, isize) {
        match self {
            Self::Up => (-1, 0),
            Self::Down => (1, 0),
            Self::Left => (0, -1),
            Self::Right => (0, 1),
        }
    }
}

fn check(
    mut lines: Array2<u8>,
    mut guard: (usize, usize),
    positions: &mut HashSet<(usize, usize)>,
) -> Result {
    // Initial direction is up
    let mut direction = Direction::Up;

    
    positions.insert(guard);
    loop {
        let direction_vector = direction.to_vector();
        let next_position = (
            guard.0 as isize + direction_vector.0,
            guard.1 as isize + direction_vector.1,
        );

        if next_position.0 < 0
            || next_position.1 < 0
            || next_position.0 >= lines.dim().0 as isize
            || next_position.1 >= lines.dim().1 as isize
        {
            break Result::Exited;
        }

        if lines[(next_position.0 as usize, next_position.1 as usize)] == b'#' {
            direction = direction.rotate();

            continue;
        }

        guard = (next_position.0 as usize, next_position.1 as usize);
        positions.insert(guard);
        if lines[(guard.0, guard.1)] == direction.to_byte() {
            break Result::InfiniteLoop;
        }
        lines[(guard.0, guard.1)] = direction.to_byte();
    }
}

fn solve(lines: impl Iterator<Item = String>) -> (usize, usize) {
    let lines = lines.collect::<Vec<_>>();

    let width = lines[0].len();
    let height = lines.len();

    let map = lines
        .into_iter()
        .flat_map(String::into_bytes)
        .collect::<Vec<_>>();

    let mut map = ndarray::Array2::from_shape_vec((height, width), map).unwrap();

    let guard = (|| {
        for i in 0..map.dim().0 {
            for j in 0..map.dim().1 {
                if map[(i, j)] == b'^' {
                    return (i, j);
                }
            }
        }
        panic!("No starting point found");
    })();

    let mut positions = HashSet::new();
    let res = check(map.clone(), guard, &mut positions);
    let part1 = positions.len();
    let positions_clone = positions.clone();
    positions.clear();

    match res {
        Result::Exited => {}
        Result::InfiniteLoop => panic!(),
    };

    let mut total_loops = 0;
    for (i, j) in positions_clone.into_iter() {
        let original = map[(i, j)];
        if original == b'#' || original == b'^' {
            continue;
        }

        map[(i, j)] = b'#';

        match check(map.clone(), guard, &mut positions) {
            Result::Exited => {}
            Result::InfiniteLoop => {
                total_loops += 1;
            }
        }
        positions.clear();

        map[(i, j)] = original;
    }

    (part1, total_loops)
}

fn main() {
    let (part1, part2) = solve(util::stdin_lines());

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
