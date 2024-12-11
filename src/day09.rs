#[derive(Clone, Copy, Debug)]
enum Block {
    Empty,
    Occupied { file_id: u32 },
}

#[derive(Clone, Copy, Debug)]
enum BlockBlock {
    Empty(usize),
    Occupied { file_id: u32, length: usize },
}

fn blockblock_to_block(expanded: Vec<BlockBlock>) -> Vec<Block> {
    expanded
        .into_iter()
        .flat_map(|block| match block {
            BlockBlock::Empty(length) => std::iter::repeat(Block::Empty).take(length),
            BlockBlock::Occupied { file_id, length } => {
                std::iter::repeat(Block::Occupied { file_id }).take(length)
            }
        })
        .collect::<Vec<_>>()
}

fn compact_part_1(expanded: Vec<BlockBlock>) -> Vec<Block> {
    let mut expanded = blockblock_to_block(expanded);

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
        }
    }

    expanded
}

fn compact_part_2(mut expanded: Vec<BlockBlock>) -> Vec<Block> {
    let max_file_id = expanded.len() / 2;
    expanded.retain(|block| !matches!(block, BlockBlock::Empty(0)));

    for file_id in (1..=max_file_id).rev() {
        let (file_idx, file) = expanded
            .iter()
            .enumerate()
            .rev()
            .find(|(_, block)| matches!(block, BlockBlock::Occupied { file_id: id, .. } if *id == file_id as u32))
            .map(|(idx, block)| (idx, *block))
            .unwrap();

        match file {
            BlockBlock::Occupied {
                file_id: _,
                length: file_length,
            } => {
                enum Res {
                    Found(usize, usize),
                    NotFound,
                }
                let success = if let Some((empty_idx, empty_block)) = expanded
                    .iter_mut()
                    .take(file_idx)
                    .enumerate()
                    .find(|(_, block)| match block {
                        BlockBlock::Empty(empty_length) => *empty_length >= file_length,
                        _ => false,
                    }) {
                    match empty_block {
                        BlockBlock::Empty(empty_length) => {
                            *empty_length -= file_length;
                            Res::Found(empty_idx, *empty_length)
                        }
                        _ => unreachable!(),
                    }
                } else {
                    Res::NotFound
                };

                if let Res::Found(empty_idx, _empty_length) = success {
                    expanded.remove(file_idx);
                    expanded.insert(file_idx, BlockBlock::Empty(file_length));
                    expanded.insert(empty_idx, file);

                    // Merge empty blocks
                    let mut leftmost_empty_idx: usize = file_idx;
                    if empty_idx < file_idx {
                        leftmost_empty_idx += 1;
                    }

                    while leftmost_empty_idx != 0
                        && matches!(
                            expanded.get(leftmost_empty_idx - 1),
                            Some(BlockBlock::Empty(_))
                        )
                    {
                        leftmost_empty_idx -= 1;
                    }
                    let leftmost_empty_idx = leftmost_empty_idx;

                    while let Some(BlockBlock::Empty(adjacent_empty_length)) =
                        expanded.get(leftmost_empty_idx + 1).copied()
                    {
                        match expanded.get_mut(leftmost_empty_idx) {
                            Some(BlockBlock::Empty(leftmost_empty_length)) => {
                                *leftmost_empty_length += adjacent_empty_length;
                            }
                            _ => unreachable!(),
                        }
                        expanded.remove(leftmost_empty_idx + 1);
                    }
                }
            }
            _ => unreachable!(),
        };
    }

    blockblock_to_block(expanded)
}

fn solve(mut lines: impl Iterator<Item = String>) -> (u64, usize) {
    let the_line = lines.next().unwrap();

    let mut expanded = Vec::new();
    let mut file_id = 0;
    for (i, character) in the_line.chars().enumerate() {
        let character_value = character.to_digit(10).unwrap();

        if i % 2 != 0 {
            expanded.push(BlockBlock::Empty(character_value as usize));
        } else {
            expanded.push(BlockBlock::Occupied {
                file_id,
                length: character_value as usize,
            });
            file_id += 1;
        }
    }

    let expanded_part_1 = compact_part_1(expanded.clone());

    let checksum: u64 = expanded_part_1
        .iter()
        .enumerate()
        .map(|(i, block)| match block {
            Block::Empty => 0,
            Block::Occupied { file_id } => *file_id as u64 * i as u64,
        })
        .sum();

    let expanded_part_2 = compact_part_2(expanded);
    let checksum_part_2: usize = expanded_part_2
        .iter()
        .enumerate()
        .map(|(i, block)| match block {
            Block::Empty => 0,
            Block::Occupied { file_id } => *file_id as usize * i,
        })
        .sum();
    (checksum, checksum_part_2)
}

fn main() {
    let (part1, part2) = solve(util::stdin_lines());

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
