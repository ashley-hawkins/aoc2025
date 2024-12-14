use itertools::Itertools;

const Y_LIMIT: i64 = 103;
const X_LIMIT: i64 = 101;

fn add_arrays<const N: usize, T: std::ops::Add<Output = T> + Copy>(
    a: &[T; N],
    b: &[T; N],
) -> [T; N] {
    std::array::from_fn(|i| a[i] + b[i])
}

fn position_after_n_steps(position: (i64, i64), velocity: (i64, i64), n: i64) -> (i64, i64) {
    (
        (position.0 + n * velocity.0).rem_euclid(X_LIMIT),
        (position.1 + n * velocity.1).rem_euclid(Y_LIMIT),
    )
}

fn solve(lines: impl Iterator<Item = String>) -> (i64, i64) {
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;

    let robots = lines
        .map(|line| -> ((i64, i64), (i64, i64)) {
            // format: "p=x,y v=x,y" e.g. "p=1,2 v=3,4"
            line.split_whitespace()
                .map(|x| -> (i64, i64) {
                    x[2..]
                        .split(",")
                        .map(|x| x.parse::<i64>().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect_tuple()
                .unwrap()
        })
        .collect_vec();

    robots.iter().for_each(|(position, velocity)| {
        let position_after_100_steps = position_after_n_steps(*position, *velocity, 100);

        match position_after_100_steps.0.cmp(&(X_LIMIT / 2)) {
            std::cmp::Ordering::Greater => match position_after_100_steps.1.cmp(&(Y_LIMIT / 2)) {
                std::cmp::Ordering::Less => {
                    q1 += 1;
                }
                std::cmp::Ordering::Greater => {
                    q2 += 1;
                }
                _ => {}
            },
            std::cmp::Ordering::Less => match position_after_100_steps.1.cmp(&(Y_LIMIT / 2)) {
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

    let part2 = (0i64..)
        .find(|steps| {
            let steps = *steps;
            let positions: Vec<[_; 2]> = robots
                .iter()
                .map(|(position, velocity)| {
                    position_after_n_steps(*position, *velocity, steps).into()
                })
                .collect_vec();

            let means = positions
                .iter()
                .fold([0, 0], |a, b| add_arrays(&a, b))
                .map(|x| x as f64 / positions.len() as f64);

            let variances = positions
                .iter()
                .map(|position| std::array::from_fn(|i| (position[i] as f64 - means[i]).powf(2.0)))
                .fold([0.0, 0.0], |a, b| add_arrays(&a, &b))
                .map(|x| x / positions.len() as f64);

            variances[0] < 500.0 && variances[1] < 500.0
        })
        .unwrap();

    (q1 * q2 * q3 * q4, part2)
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
