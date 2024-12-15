use std::{
    collections::{HashMap, HashSet},
    io,
};

use itertools::{iproduct, Itertools};
use multimap::MultiMap;

#[derive(Debug, Hash)]
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

    fn apply(&self, (row, column): (usize, usize)) -> Option<(usize, usize)> {
        match self {
            Self::Up => row.checked_sub(1).map(|row| (row, column)),
            Self::Right => Some((row, column + 1)),
            Self::Down => Some((row + 1, column)),
            Self::Left => column.checked_sub(1).map(|column| (row, column)),
        }
    }
}

#[derive(Debug)]
enum CheckClearStatus {
    Clear,
    RequiresMove((usize, usize)),
    NotClearable,
}

enum Dependencies {
    None,
    Single((usize, usize)),
    Double((usize, usize), (usize, usize)),
}

fn try_clear_space(
    grid: &mut ndarray::Array2<u8>,
    (row, column): (usize, usize),
    direction: &Direction,
) -> bool {
    let mut dependencies = HashMap::new();
    fn check_clear_space_recursive(
        grid: &ndarray::Array2<u8>,
        dependencies: &mut HashMap<(usize, usize), Dependencies>,
        (row, column): (usize, usize),
        direction: &Direction,
    ) -> CheckClearStatus {
        use CheckClearStatus::*;

        let current_character = grid[(row, column)];

        if current_character == b']' {
            return check_clear_space_recursive(
                grid,
                dependencies,
                Direction::Left.apply((row, column)).unwrap(),
                direction,
            );
        }

        if let Some(dependency) = dependencies.get(&(row, column)) {
            // If it's already in the dependencies that means that it must require a move since otherwise it would be cleared.
            if matches!(dependency, Dependencies::None) {
                return Clear;
            }
            return RequiresMove((row, column));
        }

        if current_character == b'.' {
            return Clear;
        }

        if current_character == b'[' {
            let initial_tile_1 = (row, column);
            let initial_tile_2 = Direction::Right.apply((row, column)).unwrap();
            assert!(grid[initial_tile_2] == b']', "{}", grid);

            let move_1 = match direction.apply(initial_tile_1) {
                Some(new_position) => new_position,
                None => return NotClearable,
            };

            let move_2 = match direction.apply(initial_tile_2) {
                Some(new_position) => new_position,
                None => return NotClearable,
            };

            if move_1.0 >= grid.nrows()
                || move_1.1 >= grid.ncols()
                || move_2.0 >= grid.nrows()
                || move_2.1 >= grid.ncols()
            {
                return NotClearable;
            }

            match direction {
                Direction::Up | Direction::Down => {
                    let move_1_obstacle_move =
                        check_clear_space_recursive(grid, dependencies, move_1, direction);
                    let move_2_obstacle_move =
                        check_clear_space_recursive(grid, dependencies, move_2, direction);

                    match (move_1_obstacle_move, move_2_obstacle_move) {
                        (Clear, Clear) => {
                            dependencies.insert(initial_tile_1, Dependencies::None);
                        }
                        (RequiresMove(move_), Clear) | (Clear, RequiresMove(move_)) => {
                            dependencies.insert(initial_tile_1, Dependencies::Single(move_));
                        }
                        (RequiresMove(move_1), RequiresMove(move_2)) if move_1 == move_2 => {
                            dependencies.insert(initial_tile_1, Dependencies::Single(move_1));
                        }
                        (RequiresMove(move_1), RequiresMove(move_2)) => {
                            dependencies
                                .insert(initial_tile_1, Dependencies::Double(move_1, move_2));
                        }
                        (NotClearable, _) | (_, NotClearable) => return NotClearable,
                    }
                }
                dir @ Direction::Left | dir @ Direction::Right => {
                    let single_check = match dir {
                        Direction::Left => move_1,
                        Direction::Right => move_2,
                        _ => unreachable!(),
                    };

                    let single_obstacle_move =
                        check_clear_space_recursive(grid, dependencies, single_check, direction);

                    match single_obstacle_move {
                        Clear => {
                            dependencies.insert(initial_tile_1, Dependencies::None);
                        }
                        RequiresMove(move_) => {
                            dependencies.insert(initial_tile_1, Dependencies::Single(move_));
                        }
                        NotClearable => return NotClearable,
                    }
                }
            }
            return RequiresMove(initial_tile_1);
        }

        NotClearable
    }

    let result = check_clear_space_recursive(grid, &mut dependencies, (row, column), direction);

    match result {
        CheckClearStatus::RequiresMove(coord) => {
            clear_space(grid, &dependencies, coord, direction);
        }
        CheckClearStatus::NotClearable => return false,
        _ => {}
    }

    true
}

fn clear_space(
    grid: &mut ndarray::Array2<u8>,
    dependencies: &HashMap<(usize, usize), Dependencies>,
    (row, column): (usize, usize),
    direction: &Direction,
) {
    fn clear_space_recursive(
        grid: &mut ndarray::Array2<u8>,
        dependencies: &HashMap<(usize, usize), Dependencies>,
        (row, column): (usize, usize),
        direction: &Direction,
        satisfied_dependencies: &mut HashSet<(usize, usize)>,
    ) {
        if satisfied_dependencies.contains(&(row, column)) {
            return;
        }

        if let Some(dependency) = dependencies.get(&(row, column)) {
            match dependency {
                Dependencies::None => {}
                Dependencies::Single(move_) => {
                    clear_space_recursive(
                        grid,
                        dependencies,
                        *move_,
                        direction,
                        satisfied_dependencies,
                    );
                }
                Dependencies::Double(move_1, move_2) => {
                    clear_space_recursive(
                        grid,
                        dependencies,
                        *move_1,
                        direction,
                        satisfied_dependencies,
                    );
                    clear_space_recursive(
                        grid,
                        dependencies,
                        *move_2,
                        direction,
                        satisfied_dependencies,
                    );
                }
            }
        }

        let pos_1 = (row, column);
        let pos_2 = Direction::Right.apply((row, column)).unwrap();

        let next_pos_1 = direction.apply(pos_1).unwrap();
        let next_pos_2 = direction.apply(pos_2).unwrap();

        grid[pos_1] = b'.';
        grid[pos_2] = b'.';

        grid[next_pos_1] = b'[';
        grid[next_pos_2] = b']';

        satisfied_dependencies.insert(pos_1);
    }

    clear_space_recursive(
        grid,
        dependencies,
        (row, column),
        direction,
        &mut HashSet::new(),
    );
}

fn print_grid(
    stream: &mut impl std::io::Write,
    grid: &ndarray::Array2<u8>,
    robot_pos: (usize, usize),
) {
    for row in 0..grid.nrows() {
        for column in 0..grid.ncols() {
            if (row, column) == robot_pos {
                write!(stream, "\x1b[31m@\x1b[0m").unwrap();
            } else {
                write!(stream, "{}", grid[(row, column)] as char).unwrap();
            }
        }
        println!();
    }
}

fn solve(mut lines: impl Iterator<Item = String>) -> (i64, i64) {
    let mut grid_part1 = util::lines_to_grid(lines.by_ref().take_while(|line| !line.is_empty()));

    let grid_part2 = grid_part1
        .iter()
        .flat_map(|c| *match c {
            b'@' => b"@.",
            b'#' => b"##",
            b'O' => b"[]",
            b'.' => b"..",
            _ => panic!(),
        })
        .collect::<Vec<_>>();

    let mut grid_part2 =
        ndarray::Array2::from_shape_vec((grid_part1.nrows(), grid_part1.ncols() * 2), grid_part2)
            .unwrap();

    let moves = lines
        .flat_map(|line| line.chars().map(Direction::from_char).collect_vec())
        .collect_vec();

    // Part 1
    let total_gps_1 = {
        let (mut robot_row, mut robot_column) =
            iproduct!(0..grid_part1.nrows(), 0..grid_part1.ncols())
                .find(|(row, column)| grid_part1[(*row, *column)] == b'@')
                .unwrap();

        grid_part1[(robot_row, robot_column)] = b'.';

        'outer: for move_ in &moves {
            let (robot_new_row, robot_new_column) = match move_.apply((robot_row, robot_column)) {
                Some(inner) => inner,
                None => continue,
            };

            let mut new_row = robot_row;
            let mut new_column = robot_column;

            let mut current_character;

            loop {
                (new_row, new_column) = match move_.apply((new_row, new_column)) {
                    Some(inner) => inner,
                    None => continue 'outer,
                };

                if new_row >= grid_part1.nrows() || new_column >= grid_part1.ncols() {
                    continue 'outer;
                }

                current_character = grid_part1[(new_row, new_column)];
                if current_character != b'O' {
                    break;
                }
            }

            if current_character == b'#' {
                continue;
            }

            let original = grid_part1[(robot_new_row, robot_new_column)];
            grid_part1[(robot_new_row, robot_new_column)] = b'.';
            grid_part1[(new_row, new_column)] = original;

            robot_row = robot_new_row;
            robot_column = robot_new_column;
        }

        let mut total_gps = 0;
        for row in 0..grid_part1.nrows() {
            for column in 0..grid_part1.ncols() {
                let c = grid_part1[(row, column)];
                if c == b'O' {
                    total_gps += 100 * row + column;
                }
            }
        }

        print_grid(&mut io::stdout(), &grid_part1, (robot_row, robot_column));

        total_gps
    };

    // Part 2
    let total_gps_2 = {
        let (mut robot_row, mut robot_column) =
            iproduct!(0..grid_part2.nrows(), 0..grid_part2.ncols())
                .find(|(row, column)| grid_part2[(*row, *column)] == b'@')
                .unwrap();

        grid_part2[(robot_row, robot_column)] = b'.';

        for (i, move_) in moves.iter().enumerate() {
            let (robot_new_row, robot_new_column) = match move_.apply((robot_row, robot_column)) {
                Some(inner) => inner,
                None => continue,
            };

            let check = try_clear_space(&mut grid_part2, (robot_new_row, robot_new_column), move_);

            if !check {
                continue;
            }

            robot_row = robot_new_row;
            robot_column = robot_new_column;

            #[cfg(debug_assertions)]
            for row in 0..grid_part2.nrows() {
                for column in 0..grid_part2.ncols() {
                    let c = grid_part2[(row, column)];
                    if (c == b'['
                        && grid_part2[Direction::Right.apply((row, column)).unwrap()] != b']')
                        || (c == b']'
                            && grid_part2[Direction::Left.apply((row, column)).unwrap()] != b'[')
                    {
                        let mut grid_string_writer = io::Cursor::new(Vec::new());

                        print_grid(
                            &mut grid_string_writer,
                            &grid_part2,
                            (robot_row, robot_column),
                        );

                        let grid_string =
                            String::from_utf8(grid_string_writer.into_inner()).unwrap();
                        panic!(
                            "Inconsistent state caused by move #{} at [{row}, {column}]\n{grid_string}",
                            i
                        );
                    }
                }
            }
        }

        let mut total_gps = 0;
        for row in 0..grid_part2.nrows() {
            for column in 0..grid_part2.ncols() {
                let c = grid_part2[(row, column)];
                if c == b'[' {
                    total_gps += 100 * row + column;
                }
            }
        }

        print_grid(&mut io::stdout(), &grid_part2, (robot_row, robot_column));
        total_gps
    };

    (total_gps_1 as i64, total_gps_2 as i64)
}

fn main() {
    let (part1, part2) = {
        let _stopwatch = util::ScopedStopwatch::new(|duration| {
            eprintln!("Time: {:?}", duration);
        });
        solve(util::stdin_lines())
    };

    eprintln!("Part 1: {part1}");
    eprintln!("Part 2: {part2}");
}
