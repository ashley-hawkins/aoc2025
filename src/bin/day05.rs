use util::stdin_lines;

fn solve(mut lines: impl Iterator<Item = String>) -> (usize, u64) {
    let ranges = lines
        .by_ref()
        .take_while(|x| !x.is_empty())
        .map(|line| {
            let x = <[u64; 2]>::try_from(
                line.trim()
                    .split('-')
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<_>>(),
            )
            .unwrap();
            x[0]..x[1] + 1
        })
        .collect::<Vec<_>>();

    let count = lines
        .filter(|line| {
            ranges
                .iter()
                .any(|range| range.contains(&line.parse::<u64>().unwrap()))
        })
        .count();
    (count, 0)
}

fn main() {
    let (part1, part2) = solve(stdin_lines());

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
