use std::collections::HashSet;

use itertools::Itertools;
use multimap::MultiMap;

fn solve(lines: impl Iterator<Item = String>) -> (usize, usize) {
    let mut map = MultiMap::new();

    let mut width: i64 = 0;
    let mut height: i64 = 0;
    for (y, line) in lines.enumerate() {
        let y = y as i64;
        if y == 0 {
            width = line.len() as i64;
        }
        height += 1;

        for (x, c) in line.chars().enumerate() {
            let x = x as i64;
            if c.is_alphanumeric() {
                map.insert(c, (x, y));
            }
        }
    }

    let mut antinodes: HashSet<(i64, i64)> = HashSet::new();
    let mut antinodes2: HashSet<(i64, i64)> = HashSet::new();
    for (_, positions) in map.iter_all() {
        for pair in positions.iter().combinations(2) {
            let (l, r) = <[_; 2]>::try_from(pair).unwrap().into();
            let difference1 = (2 * l.0 - r.0, 2 * l.1 - r.1);
            let difference2 = (2 * r.0 - l.0, 2 * r.1 - l.1);
            let difference3 = (l.0 - r.0, l.1 - r.1);

            let valid_x = 0..width;
            let valid_y = 0..height;

            if valid_x.contains(&difference1.0) && valid_y.contains(&difference1.1) {
                antinodes.insert(difference1);
            }

            if valid_x.contains(&difference2.0) && valid_y.contains(&difference2.1) {
                antinodes.insert(difference2);
            }

            antinodes2.insert(*l);
            antinodes2.insert(*r);
            let mut forward_positions = *r;
            while valid_x.contains(&forward_positions.0) && valid_y.contains(&forward_positions.1) {
                antinodes2.insert(forward_positions);
                forward_positions = (
                    forward_positions.0 + difference3.0,
                    forward_positions.1 + difference3.1,
                );
            }

            let mut back_positions = *r;
            while valid_x.contains(&back_positions.0) && valid_y.contains(&back_positions.1) {
                antinodes2.insert(back_positions);
                back_positions = (
                    back_positions.0 - difference3.0,
                    back_positions.1 - difference3.1,
                );
            }
        }
    }

    (antinodes.len(), antinodes2.len())
}

fn main() {
    let (part1, part2) = solve(util::stdin_lines());

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
