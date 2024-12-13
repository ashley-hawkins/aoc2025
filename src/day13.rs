use itertools::Itertools;
use util::stdin_lines;
use z3::ast::{Ast, Int};

fn solve(lines: impl Iterator<Item = String>) -> (i64, i64) {
    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);

    let a = Int::new_const(&ctx, "a".to_string());
    let b = Int::new_const(&ctx, "b".to_string());

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
            let s = z3::Optimize::new(&ctx);
            let total_cost = 3i64 * &a + &b;
            s.assert(&(a_dx * &a + b_dx * &b)._eq(&Int::from_i64(&ctx, prize_x)));
            s.assert(&(a_dy * &a + b_dy * &b)._eq(&Int::from_i64(&ctx, prize_y)));
            s.minimize(&total_cost);

            s.check(Vec::<z3::ast::Bool>::new().as_slice());
            s.get_model().map_or(0, |model| {
                model.eval(&total_cost, true).unwrap().as_i64().unwrap()
            })
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
