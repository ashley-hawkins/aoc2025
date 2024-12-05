use util::stdin_lines;

fn word_search(grid: &[Vec<u8>], coord: (usize, usize), needle: &[u8]) -> usize {
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

fn cross_mas_search(grid: &[Vec<u8>], coord: (usize, usize)) -> bool {
    if grid[coord.0][coord.1] != b'A'
        || coord.0 == 0
        || coord.1 == 0
        || coord.0 == grid.len() - 1
        || coord.1 == grid[coord.0].len() - 1
    {
        return false;
    }

    let upper_left = grid[coord.0 - 1][coord.1 - 1];
    let upper_right = grid[coord.0 - 1][coord.1 + 1];
    let lower_left = grid[coord.0 + 1][coord.1 - 1];
    let lower_right = grid[coord.0 + 1][coord.1 + 1];

    let upper_left_to_lower_right_arm =
        upper_left == b'M' && lower_right == b'S' || upper_left == b'S' && lower_right == b'M';
    let upper_right_to_lower_left_arm =
        upper_right == b'M' && lower_left == b'S' || upper_right == b'S' && lower_left == b'M';

    upper_left_to_lower_right_arm && upper_right_to_lower_left_arm
}

fn main() {
    let lines: Vec<_> = stdin_lines().map(String::into_bytes).collect();

    let part1: usize = (0..lines.len())
        .map(|i| -> usize {
            let len = lines[i].len();
            (0..len).map(|j| word_search(&lines, (i, j), b"XMAS")).sum()
        })
        .sum();

    let part2: usize = (0..lines.len())
        .map(|i| -> usize {
            let len = lines[i].len();
            (0..len)
                .map(|j| {
                    if cross_mas_search(&lines, (i, j)) {
                        1
                    } else {
                        0
                    }
                })
                .sum()
        })
        .sum();

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
