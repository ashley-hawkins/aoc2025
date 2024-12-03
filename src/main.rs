use std::io;

fn main() {
    let mut left = Vec::<i32>::new();
    let mut right = Vec::<i32>::new();
    let mut line: String = String::new();
    while match io::stdin().read_line(&mut line) {
        Ok(0) => false,
        Ok(_) => true,
        Err(_) => false,
    } {
        let mut parts = line.split_ascii_whitespace();
        left.push(parts.next().unwrap().parse().unwrap());
        right.push(parts.next().unwrap().parse().unwrap());
        line.clear();
    }

    left.sort();
    right.sort();

    let result: u32 = left
        .into_iter()
        .zip(right.into_iter())
        .map(|(l, r)| l.abs_diff(r))
        .sum();

    println!("{}", result);
}
