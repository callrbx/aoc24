use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(9);

#[derive(Debug, Clone, Copy)]
enum Block {
    File(FileBlock),
    Free,
}

#[derive(Debug, Clone, Copy)]
struct FileBlock {
    id: usize,
}

#[derive(Debug, Clone, Copy)]
enum SizedBlock {
    File(SizedFileBlock),
    Free(SizedFreeBlock),
}

#[derive(Debug, Clone, Copy)]
struct SizedFileBlock {
    id: usize,
    size: usize,
}

#[derive(Debug, Clone, Copy)]
struct SizedFreeBlock {
    size: usize,
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut blocks: VecDeque<Block> = VecDeque::with_capacity(input.len());

    let mut id = 0;
    let mut is_file = true;
    for size in input.chars() {
        if !size.is_ascii_digit() {
            break;
        }
        let length = size.to_digit(10).unwrap() as usize;
        if is_file {
            for _ in 0..length {
                blocks.push_back(Block::File(FileBlock { id }));
            }
            id += 1;
        } else {
            for _ in 0..length {
                blocks.push_back(Block::Free);
            }
        }
        is_file = !is_file;
    }

    let mut read_index = blocks.len() as isize - 1; // start from end
    let mut write_index = 0; // leftmost free space

    while read_index >= 0 {
        if let Block::File(file_block) = blocks[read_index as usize] {
            // move forward until blockfree is found
            while write_index < read_index as usize {
                if let Block::Free = blocks[write_index] {
                    break;
                }
                write_index += 1;
            }

            if write_index < read_index as usize {
                blocks[write_index] = Block::File(file_block);
                blocks[read_index as usize] = Block::Free;
                write_index += 1;
            }
        }
        read_index -= 1; // move to next fileblock from end
    }

    Some(
        blocks
            .iter()
            .enumerate()
            .filter_map(|(i, block)| match block {
                Block::File(file_block) => Some(i * file_block.id),
                Block::Free => None,
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut blocks: VecDeque<SizedBlock> = VecDeque::with_capacity(input.len());

    let mut id = 0;
    let mut is_file = true;
    for size in input.chars() {
        if !size.is_ascii_digit() {
            break;
        }
        let length = size.to_digit(10).unwrap() as usize;
        if is_file {
            blocks.push_back(SizedBlock::File(SizedFileBlock { id, size: length }));
            id += 1;
        } else {
            blocks.push_back(SizedBlock::Free(SizedFreeBlock { size: length }));
        }
        is_file = !is_file;
    }

    let mut checked_ids: HashSet<usize> = HashSet::new();
    let mut read_index = blocks.len() as isize - 1;

    while read_index >= 0 {
        let cur_file = match blocks[read_index as usize] {
            SizedBlock::File(file_block) => file_block,
            _ => {
                read_index -= 1;
                continue;
            }
        };

        if !checked_ids.insert(cur_file.id) {
            read_index -= 1;
            continue;
        }

        let mut write_index = 0;
        while write_index < blocks.len() {
            if let SizedBlock::Free(free_block) = blocks[write_index] {
                // can contain whole file
                if free_block.size >= cur_file.size && write_index < read_index as usize {
                    // move file
                    blocks[write_index] = SizedBlock::File(cur_file);
                    let mut adjust = false;

                    // reduce free size if extra
                    if free_block.size > cur_file.size {
                        blocks.insert(
                            write_index + 1,
                            SizedBlock::Free(SizedFreeBlock {
                                size: free_block.size - cur_file.size,
                            }),
                        );

                        // we insert, need to adjust
                        read_index += 1;
                        adjust = true;
                    }

                    // free original space
                    blocks[read_index as usize] = SizedBlock::Free(SizedFreeBlock {
                        size: cur_file.size,
                    });

                    // account for an insert
                    if adjust {
                        read_index += 1
                    }

                    break;
                }
            }
            write_index += 1;
        }

        read_index -= 1;
    }

    let mut checksum = 0;
    let mut current_position = 0;

    for block in blocks {
        match block {
            SizedBlock::File(file_block) => {
                for offset in 0..file_block.size {
                    checksum += (current_position + offset) * file_block.id;
                }
                current_position += file_block.size;
            }
            SizedBlock::Free(free_block) => {
                current_position += free_block.size;
            }
        }
    }

    Some(checksum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(60));

        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(132));
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
