use util::stdin_lines;

fn transform(input: i64) -> (Option<i64>, i64) {
    const RADIX: i64 = 10;
    if input == 0 {
        return (None, 1);
    }
    let digits = input.ilog(RADIX) + 1;
    if digits % 2 == 0 {
        let half_digits = 10i64.pow(digits / 2);
        let l = input / half_digits;
        let r = input % half_digits;
        return (Some(l), r);
    }

    (None, input * 2024)
}

/*
fn transform_hashmap(input: &mut HashMap<i64, u32>) {
    input.iter
}

fn solve(mut lines: impl Iterator<Item = String>) -> (i64, i64) {
    let mut the_numbers = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .fold(HashMap::new(), |mut acc, n| {
            *acc.entry(n).or_insert(0) += 1u32;
            acc
        });

    for i in 0..75 {
        transform_vec(&mut the_numbers);
        println!("{i}");
    }

    println!("{:?}", the_numbers.len());

    (0, 0)
}

*/

fn transform_vec(input: &mut Vec<i64>) {
    for i in 0..input.len() {
        let (l, r) = transform(input[i]);
        input[i] = r;

        if let Some(l) = l {
            input.push(l);
        }
    }
}

fn solve(mut lines: impl Iterator<Item = String>) -> (usize, u64) {
    let mut the_numbers = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<i64>>();

    for i in 0..25 {
        transform_vec(&mut the_numbers);
    }

    (the_numbers.len(), 0)
}

fn main() {
    let (part1, part2) = solve(stdin_lines());

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
