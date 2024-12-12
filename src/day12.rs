use std::collections::HashSet;

use util::stdin_lines;

#[derive(Clone, Copy, Debug)]
struct Metadata {
    visited: bool,
    left_edge: bool,
    right_edge: bool,
    top_edge: bool,
    bottom_edge: bool,
    left_edge_visited: bool,
    right_edge_visited: bool,
    top_edge_visited: bool,
    bottom_edge_visited: bool,
}

#[derive(Clone, Copy, Debug)]
struct Region {
    perimiter: usize,
    area: usize,
}

enum SearchResult {
    Subregion(Region),
    NotFound,
}

impl std::ops::AddAssign for Region {
    fn add_assign(&mut self, other: Self) {
        self.perimiter += other.perimiter;
        self.area += other.area;
    }
}

impl std::ops::Add for Region {
    type Output = Self;

    fn add(mut self, other: Self) -> Self {
        self += other;
        self
    }
}

struct SearchState {
    region_type: Option<u8>,
    region_points: HashSet<(usize, usize)>,
}

impl SearchState {
    fn new(x: usize, y: usize) -> Self {
        Self {
            region_type: None,
            region_points: HashSet::new(),
        }
    }
}

fn search(
    grid: &mut ndarray::Array2<(u8, Metadata)>,
    search_state: &mut SearchState,
    (x, y): (usize, usize),
) -> SearchResult {
    use SearchResult::*;
    let (ref value, metadata) = match grid.get_mut((x, y)) {
        Some(v) => v,
        None => {
            return NotFound;
        }
    };

    if let Some(region_type) = search_state.region_type {
        if *value != region_type {
            return NotFound;
        }
    }
    search_state.region_type = Some(*value);

    if metadata.visited {
        return Subregion(Region {
            perimiter: 0,
            area: 0,
        });
    }

    metadata.visited = true;

    let mut result = Region {
        perimiter: 0,
        area: 1,
    };
    search_state.region_points.insert((x, y));

    let up = (x, y.wrapping_sub(1));
    let down = (x, y.saturating_add(1));
    let left = (x.wrapping_sub(1), y);
    let right = (x.saturating_add(1), y);

    let mut check =
        |idx, idx_adj_1, idx_adj_2, meta_mapping: &dyn Fn(&mut Metadata) -> &mut bool| {
            match search(grid, search_state, idx) {
                Subregion(subregion) => {
                    result += subregion;
                }
                NotFound => {
                    *meta_mapping(&mut grid[(x, y)].1) = true;
                    // let mut check_adjacent = |idx| {
                    //     grid.get_mut(idx)
                    //         .map(|(value, metadata)| {
                    //             Some(*value) == region_type && *meta_mapping(metadata)
                    //         })
                    //         .unwrap_or(false)
                    // };

                    // if !check_adjacent(idx_adj_1) && !check_adjacent(idx_adj_2) {
                    result.perimiter += 1;
                    // }
                }
            }
        };

    check(up, left, right, &|m| &mut m.top_edge);
    check(down, left, right, &|m| &mut m.bottom_edge);
    check(left, up, down, &|m| &mut m.left_edge);
    check(right, up, down, &|m| &mut m.right_edge);

    SearchResult::Subregion(result)
}

fn solve(lines: impl Iterator<Item = String>) -> (usize, usize) {
    let grid = util::lines_to_grid(lines);
    let mut grid = grid.mapv_into_any(|value| {
        (
            value,
            Metadata {
                visited: false,
                left_edge: false,
                right_edge: false,
                top_edge: false,
                bottom_edge: false,
                left_edge_visited: false,
                right_edge_visited: false,
                top_edge_visited: false,
                bottom_edge_visited: false,
            },
        )
    });

    let mut total = 0;
    let mut regions = Vec::new();
    for x in 0..grid.shape()[0] {
        for y in 0..grid.shape()[1] {
            let mut search_state = SearchState::new(x, y);
            let region = search(&mut grid, &mut search_state, (x, y));
            if let SearchResult::Subregion(region) = region {
                if region.area > 0 {
                    total += region.area * region.perimiter;
                    regions.push((
                        search_state.region_type.unwrap(),
                        search_state.region_points,
                        region,
                    ));
                }
            }
        }
    }
    let total = total;

    let mut total_part_2 = 0;
    for (region_type, points, region) in regions {
        let mut sides: usize = 0;
        for point in points {
            let (x, y) = point;
            let metadata = grid.get_mut(point).unwrap().1;

            if metadata.top_edge && !metadata.top_edge_visited {
                grid.get_mut(point).unwrap().1.top_edge_visited = true;
                sides += 1;
                for check_x in x + 1..grid.shape()[0] {
                    let (check_val, check_metadata) = grid.get_mut((check_x, y)).unwrap();

                    if *check_val != region_type {
                        break;
                    }

                    assert!(!check_metadata.top_edge_visited);

                    if !check_metadata.top_edge {
                        break;
                    }

                    check_metadata.top_edge_visited = true;
                }
                for check_x in (0..x).rev() {
                    let (check_val, check_metadata) = grid.get_mut((check_x, y)).unwrap();

                    if *check_val != region_type {
                        break;
                    }

                    assert!(!check_metadata.top_edge_visited);

                    if !check_metadata.top_edge {
                        break;
                    }

                    check_metadata.top_edge_visited = true;
                }
            }
            if metadata.bottom_edge && !metadata.bottom_edge_visited {
                grid.get_mut(point).unwrap().1.bottom_edge_visited = true;
                sides += 1;
                for check_x in x + 1..grid.shape()[0] {
                    let (check_val, check_metadata) = grid.get_mut((check_x, y)).unwrap();

                    if *check_val != region_type {
                        break;
                    }

                    assert!(!check_metadata.bottom_edge_visited);

                    if !check_metadata.bottom_edge {
                        break;
                    }
                    check_metadata.bottom_edge_visited = true;
                }
                for check_x in (0..x).rev() {
                    let (check_val, check_metadata) = grid.get_mut((check_x, y)).unwrap();

                    if *check_val != region_type {
                        break;
                    }

                    assert!(!check_metadata.bottom_edge_visited);

                    if !check_metadata.bottom_edge {
                        break;
                    }
                    check_metadata.bottom_edge_visited = true;
                }
            }
            if metadata.left_edge && !metadata.left_edge_visited {
                grid.get_mut(point).unwrap().1.left_edge_visited = true;
                sides += 1;
                for check_y in y + 1..grid.shape()[1] {
                    let (check_val, check_metadata) = grid.get_mut((x, check_y)).unwrap();

                    if *check_val != region_type {
                        break;
                    }

                    assert!(!check_metadata.left_edge_visited);

                    if !check_metadata.left_edge {
                        break;
                    }
                    check_metadata.left_edge_visited = true;
                }
                for check_y in (0..y).rev() {
                    let (check_val, check_metadata) = grid.get_mut((x, check_y)).unwrap();

                    if *check_val != region_type {
                        break;
                    }

                    assert!(!check_metadata.left_edge_visited);

                    if !check_metadata.left_edge {
                        break;
                    }
                    check_metadata.left_edge_visited = true;
                }
            }
            if metadata.right_edge && !metadata.right_edge_visited {
                grid.get_mut(point).unwrap().1.right_edge_visited = true;
                sides += 1;
                for check_y in y + 1..grid.shape()[1] {
                    let (check_val, check_metadata) = grid.get_mut((x, check_y)).unwrap();

                    if *check_val != region_type {
                        break;
                    }

                    assert!(!check_metadata.right_edge_visited);

                    if !check_metadata.right_edge {
                        break;
                    }
                    check_metadata.right_edge_visited = true;
                }
                for check_y in (0..y).rev() {
                    let (check_val, check_metadata) = grid.get_mut((x, check_y)).unwrap();

                    if *check_val != region_type {
                        break;
                    }

                    assert!(!check_metadata.right_edge_visited);

                    if !check_metadata.right_edge {
                        break;
                    }
                    check_metadata.right_edge_visited = true;
                }
            }
        }
        total_part_2 += region.area * sides;
    }
    let total_part_2 = total_part_2;

    (total, total_part_2)
}

fn main() {
    let (part1, part2) = {
        let _stopwatch = util::ScopedStopwatch::new(|duration| {
            eprintln!("Time: {:?}", duration);
        });
        solve(stdin_lines())
    };

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
