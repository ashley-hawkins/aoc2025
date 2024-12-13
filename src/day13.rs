use good_lp::{constraint, variables, ProblemVariables, Solution, SolverModel};
use itertools::Itertools;
use util::stdin_lines;

fn solve(lines: impl Iterator<Item = String>) -> (i64, i64) {
    let result: i64 = lines
        .chunks(4)
        .into_iter()
        .map(|entry| {
            // let x = 94;
            // let y = 34;
            // let z = 22;
            // let w = 67;
            // let v = 8400;
            // let u = 5400;

            let lines = entry.collect_vec();
            let parts = lines[0].split("+").flat_map(|x| x.split(",")).collect_vec();
            let x: i32 = parts[1].trim().parse().unwrap();
            let y: i32 = parts[3].trim().parse().unwrap();
            let parts = lines[1].split("+").flat_map(|x| x.split(",")).collect_vec();
            let z: i32 = parts[1].trim().parse().unwrap();
            let w: i32 = parts[3].trim().parse().unwrap();
            let parts = lines[2].split("=").flat_map(|x| x.split(",")).collect_vec();
            let v: i32 = parts[1].trim().parse().unwrap();
            let u: i32 = parts[3].trim().parse().unwrap();

            drop(lines);

            variables! {
                vars:
                    0 <= a (integer) <= 100;
                    0 <= b (integer) <= 100;
            }

            let mut problem = vars.minimise(3 * a + b).using(good_lp::default_solver);
            problem.set_parameter("log", "0");
            problem
                .with(constraint!((x * a + z * b) == v))
                .with(constraint!((y * a + w * b) == u))
                .solve()
                .map_or(0, |solution| {
                    let a = solution.value(a).round();
                    let b = solution.value(b).round();

                    (3.0 * a + b) as i64
                })
        })
        .sum();

    (result, 0)
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
