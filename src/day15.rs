use itertools::{iproduct, Itertools};

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn from_char(c: char) -> Self {
        match c {
            '^' => Self::Up,
            '>' => Self::Right,
            'v' | 'V' => Self::Down,
            '<' => Self::Left,
            _ => panic!("Invalid direction: {}", c),
        }
    }

    fn into_tuple(self) -> (i64, i64) {
        match self {
            Self::Up => (-1, 0),
            Self::Right => (0, 1),
            Self::Down => (1, 0),
            Self::Left => (0, -1),
        }
    }
}

fn solve(mut lines: impl Iterator<Item = String>) -> (i64, i64) {
    let mut grid = util::lines_to_grid(lines.by_ref().take_while(|line| !line.is_empty()));
    let moves = lines
        .flat_map(|line| line.chars().map(Direction::from_char).collect_vec())
        .collect_vec();

    let (mut row, mut column) = iproduct!(0..grid.nrows(), 0..grid.ncols())
        .find(|(row, column)| grid[(*row, *column)] == b'@')
        .unwrap();

    grid[(row, column)] = b'.';

    'outer: for move_ in moves {
        let (delta_row, delta_column) = move_.into_tuple();

        let robot_new_row = (row as i64 + delta_row) as usize;
        let robot_new_column: usize = (column as i64 + delta_column) as usize;

        let mut new_row = row as i64;
        let mut new_column = column as i64;

        let mut current_character;

        let mut count = 0;
        loop {
            new_row += delta_row;
            new_column += delta_column;

            if new_row < 0
                || new_row >= grid.nrows() as i64
                || new_column < 0
                || new_column >= grid.ncols() as i64
            {
                continue 'outer;
            }

            current_character = grid[(new_row as usize, new_column as usize)];
            if current_character != b'O' {
                break;
            }
        }

        if current_character == b'#' {
            continue;
        }

        let original = grid[(robot_new_row, robot_new_column)];
        grid[(robot_new_row, robot_new_column)] = b'.';
        grid[(new_row as usize, new_column as usize)] = original;

        row = robot_new_row;
        column = robot_new_column;
    }

    let mut total_gps = 0;
    for row in 0..grid.nrows() {
        for column in 0..grid.ncols() {
            let c = grid[(row, column)];
            if c == b'O' {
                total_gps += 100 * row + column;
            }
            print!("{}", c as char);
        }
        println!();
    }

    (total_gps as i64, 0)
}

fn main() {
    let (part1, part2) = {
        let _stopwatch = util::ScopedStopwatch::new(|duration| {
            eprintln!("Time: {:?}", duration);
        });
        solve(util::stdin_lines())
    };

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
