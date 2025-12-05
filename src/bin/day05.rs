use util::stdin_lines;

fn solve(mut lines: impl Iterator<Item = String>) -> (usize, usize) {
    let mut ranges = lines
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

    ranges.sort_by_key(|r| r.start);

    let mut non_overlapping_ranges = vec![ranges[0].clone()];

    for rng in &ranges[1..] {
        let last_rng = non_overlapping_ranges.last_mut().unwrap();
        if rng.start <= last_rng.end {
            last_rng.end = last_rng.end.max(rng.end);
        } else {
            non_overlapping_ranges.push(rng.clone());
        }
    }

    let count = lines
        .filter(|line| {
            non_overlapping_ranges
                .iter()
                .any(|range| range.contains(&line.parse::<u64>().unwrap()))
        })
        .count();

    let count2: usize = non_overlapping_ranges.into_iter().map(|rng| rng.count()).sum();

    (count, count2)
}

fn main() {
    let (part1, part2) = solve(stdin_lines());

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
