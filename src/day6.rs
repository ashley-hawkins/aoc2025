use std::collections::HashSet;

enum Result {
    Exited(usize),
    InfiniteLoop,
}

fn check(lines: &[Vec<u8>], mut guard: (usize, usize)) -> Result {
    // Initial direction is up
    let mut direction = (-1, 0);

    let mut positions = HashSet::new();
    let mut movements = HashSet::new();
    positions.insert(guard);
    loop {
        let next_position = (
            guard.0 as isize + direction.0,
            guard.1 as isize + direction.1,
        );

        if next_position.0 < 0
            || next_position.1 < 0
            || next_position.0 >= lines.len() as isize
            || next_position.1 >= lines[next_position.0 as usize].len() as isize
        {
            break Result::Exited(positions.len());
        }

        if lines[next_position.0 as usize][next_position.1 as usize] == b'#' {
            direction = match direction {
                (-1, 0) => (0, 1),
                (0, 1) => (1, 0),
                (1, 0) => (0, -1),
                (0, -1) => (-1, 0),
                _ => panic!("Invalid direction"),
            };

            continue;
        }

        guard = (next_position.0 as usize, next_position.1 as usize);
        positions.insert(guard);
        if movements.contains(&(guard, direction)) {
            break Result::InfiniteLoop;
        }
        movements.insert((guard, direction));
    }
}

fn solve(lines: impl Iterator<Item = String>) -> (usize, usize) {
    let mut lines: Vec<_> = lines.map(String::into_bytes).collect();

    let mut guard = (|| {
        for i in 0..lines.len() {
            for j in 0..lines[i].len() {
                if lines[i][j] == b'^' {
                    return (i, j);
                }
            }
        }
        panic!("No starting point found");
    })();

    let positions = match check(&lines, guard) {
        Result::Exited(positions) => positions,
        Result::InfiniteLoop => panic!(),
    };

    let mut total_loops = 0;
    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            let original = lines[i][j];
            if original == b'#' || original == b'^' {
                continue;
            }

            lines[i][j] = b'#';

            match check(&lines, guard) {
                Result::Exited(_) => {}
                Result::InfiniteLoop => {
                    total_loops += 1;
                }
            }

            lines[i][j] = original;
        }
        println!("{i}/{}", lines.len());
    }

    (positions, total_loops)
}

fn main() {
    let (part1, part2) = solve(util::stdin_lines());

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
