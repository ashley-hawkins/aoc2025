use itertools::Itertools;
use util::stdin_lines;

fn solve_with_cramers_rule(
    button_a_dx: i64,
    button_a_dy: i64,
    button_b_dx: i64,
    button_b_dy: i64,
    prize_x: i64,
    prize_y: i64,
) -> Option<(i64, i64)> {
    // Matrices:
    // | a_dx b_dx | | a | = | prize_x |
    // | a_dy b_dy | | b | = | prize_y |

    // Determinant of the coefficients matrix:
    // | a_dx b_dx |
    // | a_dy b_dy |
    let det_coefficients = (button_a_dx * button_b_dy) - (button_a_dy * button_b_dx);

    // This means there isn't one unique solution.
    // This edge case does not need to be handled, so return None.
    if det_coefficients == 0 {
        return None;
    }

    // Determinant of the a matrix:
    // | prize_x b_dx |
    // | prize_y b_dy |
    let det_a = (prize_x * button_b_dy) - (prize_y * button_b_dx);

    // Determinant of the b matrix:
    // | a_dx prize_x |
    // | a_dy prize_y |
    let det_b = (button_a_dx * prize_y) - (button_a_dy * prize_x);

    let solution_a = det_a as f64 / det_coefficients as f64;
    let solution_b = det_b as f64 / det_coefficients as f64;

    // Only integer solutions are desired.
    (solution_a == solution_a.floor() && solution_b == solution_b.floor())
        .then_some((solution_a as i64, solution_b as i64))
}

fn solve(lines: impl Iterator<Item = String>) -> (i64, i64) {
    let mut part1: i64 = 0;
    let mut part2: i64 = 0;
    for entry in lines.chunks(4).into_iter() {
        let lines = entry.collect_vec();
        let parts = lines[0].split("+").flat_map(|x| x.split(",")).collect_vec();
        let a_dx: i64 = parts[1].trim().parse().unwrap();
        let a_dy: i64 = parts[3].trim().parse().unwrap();
        let parts = lines[1].split("+").flat_map(|x| x.split(",")).collect_vec();
        let b_dx: i64 = parts[1].trim().parse().unwrap();
        let b_dy: i64 = parts[3].trim().parse().unwrap();
        let parts = lines[2].split("=").flat_map(|x| x.split(",")).collect_vec();
        let prize_x_1: i64 = parts[1].trim().parse().unwrap();
        let prize_y_1: i64 = parts[3].trim().parse().unwrap();
        let prize_x_2 = prize_x_1 + 10000000000000;
        let prize_y_2 = prize_y_1 + 10000000000000;

        let minimise_with_prize_position = |prize_x, prize_y| {
            solve_with_cramers_rule(a_dx, a_dy, b_dx, b_dy, prize_x, prize_y)
                .map(|(a, b)| 3 * a + b)
                .unwrap_or(0)
        };

        part1 += minimise_with_prize_position(prize_x_1, prize_y_1);
        part2 += minimise_with_prize_position(prize_x_2, prize_y_2);
    }

    (part1, part2)
}

fn main() {
    let (part1, part2) = {
        let _stopwatch = util::ScopedStopwatch::new(|duration| {
            eprintln!("Time: {:?}", duration);
        });
        solve(stdin_lines())
    };

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
