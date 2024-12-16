use std::collections::{BinaryHeap, HashMap, HashSet};

use itertools::iproduct;
use multimap::MultiMap;

#[repr(u8)]
#[derive(Debug, Hash, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn apply(&self, (row, column): (usize, usize)) -> Option<(usize, usize)> {
        match self {
            Self::North => row.checked_sub(1).map(|row| (row, column)),
            Self::East => Some((row, column + 1)),
            Self::South => Some((row + 1, column)),
            Self::West => column.checked_sub(1).map(|column| (row, column)),
        }
    }

    fn from_char(c: char) -> Self {
        match c {
            '^' => Self::North,
            '>' => Self::East,
            'v' | 'V' => Self::South,
            '<' => Self::West,
            _ => panic!("Invalid direction: {}", c),
        }
    }

    fn into_char(&self) -> char {
        match self {
            Self::North => '^',
            Self::East => '>',
            Self::South => 'v',
            Self::West => '<',
        }
    }

    fn min_turns_to(&self, other: &Self) -> i64 {
        match other {
            val if val == self => 0,
            Self::North => match self {
                Self::South => 2,
                _ => 1,
            },
            Self::East => match self {
                Self::West => 2,
                _ => 1,
            },
            Self::South => match self {
                Self::North => 2,
                _ => 1,
            },
            Self::West => match self {
                Self::East => 2,
                _ => 1,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
struct Node {
    position: (usize, usize),
    direction: Direction,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct State {
    cost: i64,
    node: Node,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| other.node.cmp(&self.node))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Metadata {
    cost: i64,
    visited: bool,
}

impl Metadata {
    fn new() -> Self {
        Self {
            cost: i64::MAX,
            visited: false,
        }
    }
}

const FORWARD_COST: i64 = 1;
const TURN_COST: i64 = 1000;

// Part 2 idea:
// Store "previous" as a MultiMap<Node, Node> and as a post process step, check every node in the multimap against its cost and eliminate the non-optimal ones. That way we can get every possible path and its cost.

fn solve(lines: impl Iterator<Item = String>) -> (i64, i64) {
    let maze: ndarray::Array2<u8> = util::lines_to_grid(lines);

    let (start_row, start_col) = iproduct!(0..maze.nrows(), 0..maze.ncols())
        .find(|&(r, c)| maze[(r, c)] == b'S')
        .unwrap();
    let (end_row, end_col) = iproduct!(0..maze.nrows(), 0..maze.ncols())
        .find(|&(r, c)| maze[(r, c)] == b'E')
        .unwrap();

    let mut metadata = HashMap::<Node, Metadata>::new();
    let mut heap = BinaryHeap::new();

    let mut prevs = MultiMap::<Node, Node>::new();

    heap.push(State {
        cost: 0,
        node: Node {
            position: (start_row, start_col),
            direction: Direction::East,
        },
    });

    let mut shortest_vec = Vec::new();
    while let Some(state) = heap.pop() {
        let meta = metadata.entry(state.node).or_insert_with(Metadata::new);

        if meta.visited {
            continue;
        }
        meta.visited = true;

        if state.node.position == (end_row, end_col) {
            shortest_vec.push((state.cost, state.node));
        }

        for new_direction in &[
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ] {
            let new_position = new_direction.apply(state.node.position).unwrap();
            let new_node = Node {
                position: new_position,
                direction: *new_direction,
            };

            let turn_penalty = TURN_COST * state.node.direction.min_turns_to(new_direction);
            let new_cost = state.cost + turn_penalty + FORWARD_COST;

            let new_meta = metadata.entry(new_node).or_insert_with(Metadata::new);

            if new_meta.cost >= new_cost {
                if maze[new_node.position] == b'#' {
                    continue;
                }

                heap.push(State {
                    cost: new_cost,
                    node: new_node,
                });

                if new_meta.cost != new_cost {
                    if let Some(v) = prevs.get_vec_mut(&new_node) {
                        v.clear()
                    }
                }

                new_meta.cost = new_cost;
                prevs.insert(new_node, state.node);
            }
        }
    }

    let shortest = shortest_vec.iter().map(|(cost, _)| *cost).min().unwrap();
    shortest_vec.retain(|(cost, _)| *cost == shortest);

    let shortest_set: HashSet<Node> = shortest_vec.iter().map(|(_, node)| *node).collect();

    // println!(
    //     "{:?} and {:?} / {:?} / {:?} / {:?}",
    //     prevs
    //         .get_vec(&Node {
    //             position: (1, 6),
    //             direction: Direction::East,
    //         })
    //         .unwrap(),
    //     metadata[&Node {
    //         position: (1, 5),
    //         direction: Direction::East
    //     }],
    //     metadata[&Node {
    //         position: (1, 5),
    //         direction: Direction::North
    //     }],
    //     metadata[&Node {
    //         position: (6, 1),
    //         direction: Direction::North
    //     }],
    //     metadata[&Node {
    //         position: (2, 5),
    //         direction: Direction::North
    //     }],
    // );

    let mut all_shortest_positions = HashSet::new();
    fn recursively_traverse_prevs(
        prevs: &MultiMap<Node, Node>,
        node: Node,
        all_shortest_positions: &mut HashSet<(usize, usize)>,
        visited: &mut HashSet<Node>,
    ) {
        if visited.contains(&node) {
            return;
        }
        visited.insert(node);

        all_shortest_positions.insert(node.position);
        if prevs.contains_key(&node) {
            let vec = prevs.get_vec(&node).unwrap();
            if vec.is_empty() {
                return;
            }

            for prev in vec {
                recursively_traverse_prevs(prevs, *prev, all_shortest_positions, visited);
            }
        }
    }

    // println!(
    //     "{:?}",
    //     prevs
    //         .get_vec(&Node {
    //             position: (1, 6),
    //             direction: Direction::East,
    //         })
    //         .unwrap()
    // );

    println!("Shortest path: {}", shortest);

    for node in shortest_set {
        recursively_traverse_prevs(
            &prevs,
            node,
            &mut all_shortest_positions,
            &mut HashSet::new(),
        );
    }

    for row in 0..maze.nrows() {
        for col in 0..maze.ncols() {
            let c = if all_shortest_positions.contains(&(row, col)) {
                'O'
            } else if (row, col) == (start_row, start_col) {
                'S'
            } else if (row, col) == (end_row, end_col) {
                'E'
            } else {
                maze[(row, col)] as char
            };
            print!("{}", c);
        }
        println!();
    }

    (shortest, all_shortest_positions.len() as i64)
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
