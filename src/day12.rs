use util::stdin_lines;

struct Metadata {
    visited: bool,
}

#[derive(Clone, Copy, Debug)]
struct Region {
    perimiter: usize,
    area: usize,
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

fn search(
    grid: &mut ndarray::Array2<(u8, Metadata)>,
    mut region_type: Option<u8>,
    x: usize,
    y: usize,
) -> Region {
    let (ref value, metadata) = match grid.get_mut((x, y)) {
        Some(v) => v,
        None => {
            return Region {
                perimiter: 1,
                area: 0,
            }
        }
    };

    if let Some(region_type) = region_type {
        if *value != region_type {
            return Region {
                perimiter: 1,
                area: 0,
            };
        }
    }
    region_type = Some(*value);

    if metadata.visited {
        return Region {
            perimiter: 0,
            area: 0,
        };
    }

    metadata.visited = true;

    let mut result = Region {
        perimiter: 0,
        area: 1,
    };

    result += search(grid, region_type, x.wrapping_sub(1), y);
    result += search(grid, region_type, x, y.wrapping_sub(1));
    result += search(grid, region_type, x.saturating_add(1), y);
    result += search(grid, region_type, x, y.saturating_add(1));

    result
}

fn solve(lines: impl Iterator<Item = String>) -> (usize, i64) {
    let grid = util::lines_to_grid(lines);
    let mut grid = grid.mapv_into_any(|value| (value, Metadata { visited: false }));

    let mut total = 0;
    for x in 0..grid.shape()[0] {
        for y in 0..grid.shape()[1] {
            let region = search(&mut grid, None, x, y);
            total += region.area * region.perimiter;
        }
    }

    (total, 0)
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
