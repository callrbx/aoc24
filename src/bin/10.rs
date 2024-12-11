advent_of_code::solution!(10);

use pathfinding::prelude::{bfs_reach, count_paths, Matrix};

pub fn part_one(input: &str) -> Option<usize> {
    let grid =
        Matrix::square_from_vec(input.chars().filter_map(|c| c.to_digit(10)).collect()).unwrap();

    let get_neighbors = |&(x, y): &(usize, usize)| -> Vec<(usize, usize)> {
        let cur = match grid.get((x, y)) {
            Some(v) => v,
            None => return vec![],
        };

        grid.neighbours((x, y), false)
            .filter(|&x| Some(cur + 1) == grid.get(x).copied())
            .collect()
    };

    let total_paths = grid
        .items()
        .map(|((r, c), &v)| {
            if v == 0 {
                bfs_reach((r, c), get_neighbors)
                    .filter(|&(x, y)| grid.get((x, y)) == Some(&9))
                    .count()
            } else {
                0
            }
        })
        .sum();

    Some(total_paths)
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid =
        Matrix::square_from_vec(input.chars().filter_map(|c| c.to_digit(10)).collect()).unwrap();

    let get_neighbors = |&(x, y): &(usize, usize)| -> Vec<(usize, usize)> {
        let cur = match grid.get((x, y)) {
            Some(v) => v,
            None => return vec![],
        };

        grid.neighbours((x, y), false)
            .filter(|&x| Some(cur + 1) == grid.get(x).copied())
            .collect()
    };

    let total_paths = grid
        .items()
        .map(|((r, c), &v)| {
            if v == 0 {
                count_paths((r, c), get_neighbors, |&idx| grid.get(idx) == Some(&9))
            } else {
                0
            }
        })
        .sum();

    Some(total_paths)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
