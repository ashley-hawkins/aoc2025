use util::stdin_lines;

fn word_search(
    grid: &[Vec<u8>],
    coord: (usize, usize),
    needle: &[u8],
) -> usize {
    if grid[coord.0][coord.1] != needle[0] {
        return 0;
    }

    let mut sum = 0;
    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }

            for k in 1..needle.len() {
                let x = coord.0 as isize + i * k as isize;
                let y = coord.1 as isize + j * k as isize;

                if x < 0 || y < 0 {
                    break;
                }

                let x = x as usize;
                let y = y as usize;

                if x >= grid.len() || y >= grid[x].len() {
                    break;
                }

                if grid[x][y] != needle[k] {
                    break;
                }

                if k == needle.len() - 1 {
                    sum += 1;
                }
            }
        }
    }

    sum
}

fn main() {
    let lines: Vec<_> = stdin_lines().map(String::into_bytes).collect();

    let sum: usize = (0..lines.len())
        .map(|i| -> usize {
            let len = lines[i].len();
            println!();
            (0..len)
                .map(|j| {
                    let res = word_search(&lines, (i, j), b"XMAS");
                    print!("{res} ");
                    res
                })
                .sum()
        })
        .sum();

    println!("Part 1: {}", sum);
}
