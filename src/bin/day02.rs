use util::stdin_split;

fn check_invalid(num: u64, part_2: bool) -> bool {
    let digits = num.ilog10() as usize + 1;

    let range = if part_2 { 2..=digits } else { 2..=2 };

    'outer: for i in range {
        if !digits.is_multiple_of(i) {
            continue;
        }

        let increment = digits / i;

        let denom = 10u64.pow(increment as u32);
        let reference = num % denom;

        for j in 1..i {
            let next = num / 10u64.pow((increment * j) as u32);

            if reference != next % denom {
                continue 'outer;
            }
        }
        return true;
    }

    false
}

fn solve(lines: impl Iterator<Item = String>) -> (u64, u64) {
    let ids: Vec<_> = lines
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
        .collect();

    let count = ids.iter().filter(|&&num| check_invalid(num, false)).sum();
    let count2 = ids.iter().filter(|&&num| check_invalid(num, true)).sum();

    (count, count2)
}

fn main() {
    let (part1, part2) = solve(stdin_split(b','));

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
