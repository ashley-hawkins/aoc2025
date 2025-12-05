use util::stdin_split;

fn solve(lines: impl Iterator<Item = String>) -> (u64, u64) {
    let mut count: u64 = lines
        .flat_map(|line| {
            let (begin, end) = <[u64; 2]>::try_from(
                line.split('-')
                    .map(|num| num.trim().parse::<u64>().unwrap())
                    .collect::<Vec<_>>(),
            )
            .unwrap()
            .into();

            begin..end + 1
        })
        .filter(|num| {
            let digits = num.ilog10() as usize + 1;

            if !digits.is_multiple_of(2) {
                return false;
            }

            let half = digits / 2;
            let denom = 10u64.pow(half as u32);

            let invalid = num % denom == num / denom;

            invalid
        })
        .sum();

    (count, 0)
}

fn main() {
    let (part1, part2) = solve(stdin_split(b','));

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
