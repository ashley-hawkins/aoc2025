use util::stdin_lines;

fn solve(lines: impl Iterator<Item = String>) -> (u32, u32) {
    let mut dial: i32 = 50;
    let mut count: u32 = 0;
    let mut count2: u32 = 0;

    for line in lines {
        let mut chars = line.chars();
        let direction: i32 = if chars.next().unwrap() == 'L' { -1 } else { 1 };

        let amount: i32 = direction * chars.as_str().parse::<i32>().unwrap();

        let dial_was_zero = dial == 0;
        dial += amount;

        count2 += (dial / 100).unsigned_abs();
        if dial <= 0 && !dial_was_zero {
            count2 += 1;
        }

        dial = dial.rem_euclid(100);

        if dial == 0 {
            count += 1;
        }
    }

    (count, count2)
}

fn main() {
    let (part1, part2) = solve(stdin_lines());

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
