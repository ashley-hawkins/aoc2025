use std::collections::HashSet;

use util::stdin_lines;

fn solve(lines: impl Iterator<Item = String>) -> (i64, i64) {
    let mut map = util::lines_to_grid(lines);

    fn rank_trailhead_recursive(
        map: &ndarray::Array2<u8>,
        all_ends: &mut HashSet<(usize, usize)>,
        n: u32,
        (x, y): (usize, usize),
    ) {
        if let Some(value) = (map[(x, y)] as char).to_digit(10) {
            if n == value {
                if n == 9 {
                    all_ends.insert((x, y));
                    return;
                }
            } else {
                return;
            }
        } else {
            return;
        }

        for (dx, dy) in &[(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let new_x = x as isize + dx;
            let new_y = y as isize + dy;

            if new_x < 0
                || new_y < 0
                || new_x >= map.dim().0 as isize
                || new_y >= map.dim().1 as isize
            {
                continue;
            }

            let new_x = new_x as usize;
            let new_y = new_y as usize;

            rank_trailhead_recursive(map, all_ends, n + 1, (new_x, new_y));
        }
    }
    let rank_trailhead = |(x, y)| {
        let mut ends = Default::default();
        rank_trailhead_recursive(&map, &mut ends, 0, (x, y));
        ends.len()
    };

    let mut total = 0;
    for i in 0..map.dim().0 {
        for j in 0..map.dim().1 {
            let score = rank_trailhead((i, j));
            if score > 0 {
                println!("{i} {j}: {score}");
                total += score;
            }
        }
    }

    (total as i64, 0)
}

fn main() {
    let (part1, part2) = solve(stdin_lines());

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
