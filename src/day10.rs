use std::collections::HashSet;

use util::stdin_lines;

fn solve(lines: impl Iterator<Item = String>) -> (i64, i64) {
    let map = util::lines_to_grid(lines);

    fn rank_trailhead_recursive(
        map: &ndarray::Array2<u8>,
        endpoints: &mut HashSet<(usize, usize)>,
        n: u32,
        (x, y): (usize, usize),
    ) -> usize {
        if let Some(value) = (map[(x, y)] as char).to_digit(10) {
            if n == value {
                if n == 9 {
                    endpoints.insert((x, y));
                    return 1;
                }
            } else {
                return 0;
            }
        }

        let mut total: usize = 0;
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

            total += rank_trailhead_recursive(map, endpoints, n + 1, (new_x, new_y));
        }
        total
    }
    let rank_trailhead = |(x, y)| {
        let mut endpoints = Default::default();
        let part2 = rank_trailhead_recursive(&map, &mut endpoints, 0, (x, y));
        (endpoints.len(), part2)
    };

    let mut part1 = 0;
    let mut part2 = 0;
    for i in 0..map.dim().0 {
        for j in 0..map.dim().1 {
            let (rank1, rank2) = rank_trailhead((i, j));
            part1 += rank1;
            part2 += rank2;
        }
    }

    (part1 as i64, part2 as i64)
}

fn main() {
    let (part1, part2) = solve(stdin_lines());

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
