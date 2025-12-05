use util::{lines_to_grid, stdin_lines};

fn solve(mut grid: ndarray::Array2<u8>) -> (u64, u64) {
    let mut part1 = 0;
    let mut part2 = 0;
    for i in 0usize.. {
        let mut this_pass = 0;
        let grid_snapshot = grid.clone();

        for row in 0..grid.dim().0 {
            for col in 0..grid.dim().1 {
                if grid_snapshot[(row, col)] != b'@' {
                    continue;
                }

                let mut count = 0;
                for i in -1..=1 {
                    for j in -1..=1 {
                        if let Some(new_row) = (row as isize).checked_add(i)
                            && let Some(new_col) = (col as isize).checked_add(j)
                        {
                            let new_row = new_row as usize;
                            let new_col = new_col as usize;
                            if grid_snapshot.get((new_row, new_col)) == Some(&b'@') {
                                count += 1;
                            }
                        }
                    }
                }

                if count <= 4 {
                    this_pass += 1;
                    grid[(row, col)] = b'.';
                }
            }
        }
        if this_pass == 0 {
            break;
        }

        if i == 0 {
            part1 = this_pass;
        }
        part2 += this_pass;
    }
    (part1, part2)
}

fn main() {
    let (part1, part2) = solve(lines_to_grid(stdin_lines()));

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
