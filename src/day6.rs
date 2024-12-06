use std::collections::HashSet;

fn solve(lines: impl Iterator<Item = String>) -> (usize, usize) {
    let lines: Vec<_> = lines.map(String::into_bytes).collect();

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

    // Initial direction is up
    let mut direction = (-1, 0);

    let mut positions = HashSet::new();
    positions.insert(guard);
    loop {
        let next_position = (guard.0 as isize + direction.0, guard.1 as isize + direction.1);
        if next_position.0 < 0 || next_position.1 < 0 || next_position.0 >= lines.len() as isize || next_position.1 >= lines[next_position.0 as usize].len() as isize {
            break;
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
    }

    (positions.len(), 0)
}

fn main() {
    let (part1, part2) = solve(util::stdin_lines());

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
