use itertools::Itertools;

fn solve(lines: impl Iterator<Item = String>) -> (i64, i64) {
    const Y_LIMIT: i64 = 103;
    const X_LIMIT: i64 = 101;
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;

    lines.for_each(|line| {
        // format: "p=x,y v=x,y" e.g. "p=1,2 v=3,4"
        let (position, velocity) = line
            .split_whitespace()
            .map(|x| -> (i64, i64) {
                x[2..]
                    .split(",")
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .collect_tuple()
            .unwrap();

        let next_position_after_100_steps = (
            (position.0 + 100 * velocity.0).rem_euclid(X_LIMIT),
            (position.1 + 100 * velocity.1).rem_euclid(Y_LIMIT),
        );

        match next_position_after_100_steps.0.cmp(&(X_LIMIT / 2)) {
            std::cmp::Ordering::Greater => {
                match next_position_after_100_steps.1.cmp(&(Y_LIMIT / 2)) {
                    std::cmp::Ordering::Less => {
                        q1 += 1;
                    }
                    std::cmp::Ordering::Greater => {
                        q2 += 1;
                    }
                    _ => {}
                }
            }
            std::cmp::Ordering::Less => match next_position_after_100_steps.1.cmp(&(Y_LIMIT / 2)) {
                std::cmp::Ordering::Greater => {
                    q3 += 1;
                }
                std::cmp::Ordering::Less => {
                    q4 += 1;
                }
                _ => {}
            },
            _ => {}
        }
    });

    (q1 * q2 * q3 * q4, 0)
}

fn main() {
    let (part1, part2) = {
        let _stopwatch = util::ScopedStopwatch::new(|duration| {
            eprintln!("Time: {:?}", duration);
        });
        solve(util::stdin_lines())
    };

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
