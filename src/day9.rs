#[derive(Clone, Copy, Debug)]
enum Block {
    Empty,
    Occupied { file_id: u32 },
}

fn solve(mut lines: impl Iterator<Item = String>) -> (u64, usize) {
    let the_line = lines.next().unwrap();
    let mut expanded = Vec::new();

    let mut file_id = 0;
    for (i, character) in the_line.chars().enumerate() {
        let character_value = character.to_digit(10).unwrap();

        if i % 2 != 0 {
            expanded.extend(std::iter::repeat(Block::Empty).take(character_value as usize));
        } else {
            expanded.extend(
                std::iter::repeat(Block::Occupied { file_id }).take(character_value as usize),
            );
            file_id += 1;
        }
    }

    // println!("{}", expanded.iter().map(|block| match block {
    //     Block::Empty => '.',
    //     Block::Occupied { file_id } => std::char::from_digit(*file_id % 10, 10).unwrap(),
    // }).collect::<String>());
    let mut start_point = expanded.len() - 1;
    for block_index in 0..expanded.len() {
        if block_index >= start_point {
            break;
        }
        if matches!(expanded[block_index], Block::Empty) {
            let mut swapped = false;
            for other_block_index in ((block_index + 1)..=start_point).rev() {
                if matches!(expanded[other_block_index], Block::Occupied { .. }) {
                    expanded.swap(block_index, other_block_index);
                    start_point = other_block_index - 1;
                    swapped = true;
                    break;
                }
            }
            if !swapped {
                break;
            }
            // println!("{}", expanded.iter().map(|block| match block {
            //     Block::Empty => '.',
            //     Block::Occupied { file_id } => std::char::from_digit(*file_id % 10, 10).unwrap(),
            // }).collect::<String>());
        }
    }

    let checksum: u64 = expanded
        .iter()
        .enumerate()
        .map(|(i, block)| match block {
            Block::Empty => 0,
            Block::Occupied { file_id } => *file_id as u64 * i as u64,
        })
        .sum();

    (checksum, 0)
}

fn main() {
    let (part1, part2) = solve(util::stdin_lines());

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
