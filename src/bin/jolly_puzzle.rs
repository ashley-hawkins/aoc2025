use util::stdin_lines;

fn solve(mut lines: impl Iterator<Item = String>) -> Vec<u64> {
    let count = lines
        .by_ref()
        .next()
        .unwrap()
        .trim()
        .parse::<usize>()
        .unwrap();
    let mut results = vec![0u64; count];

    for line in lines {
        let nums = line
            .trim()
            .split_ascii_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let idx = nums[0];
        let base = nums[1];
        let exponent = nums[2];
        let mut result = base.pow(exponent as u32);
        let mut real_result = 0;
        while result > 0 {
            real_result *= 10;
            real_result += result % 10;
            result /= 10;
        }
        results[idx] = real_result as u64;
    }

    results
}

fn main() {
    let res = solve(stdin_lines());

    for val in res {
        println!("{val}");
    }
}
