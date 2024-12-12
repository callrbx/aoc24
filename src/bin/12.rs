advent_of_code::solution!(12);

use pathfinding::prelude::{bfs_reach, Matrix};
use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<usize> {
    let grid =
        Matrix::square_from_vec(input.chars().filter(|c| !c.is_ascii_whitespace()).collect())
            .unwrap();

    let get_neighbors = |&(x, y): &(usize, usize)| -> Vec<(usize, usize)> {
        let cur = match grid.get((x, y)) {
            Some(v) => v,
            None => return vec![],
        };

        grid.neighbours((x, y), false)
            .filter(|&x| Some(cur) == grid.get(x))
            .collect()
    };

    let mut found: HashSet<(usize, usize)> = HashSet::new();

    Some(grid.items().fold(0, |mut total_price, ((r, c), _)| {
        // skip if already in a region
        if found.contains(&(r, c)) {
            return total_price;
        }

        // isolate region
        let region: Vec<(usize, usize)> = bfs_reach((r, c), get_neighbors)
            .inspect(|&coord| {
                found.insert(coord);
            })
            .collect();

        // get perimeter for each coordinate: 4 possible perms - number of internal faces
        for coord in &region {
            total_price += region.len() * (4 - get_neighbors(coord).len());
        }

        total_price
    }))
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid =
        Matrix::square_from_vec(input.chars().filter(|c| !c.is_ascii_whitespace()).collect())
            .unwrap();

    let get_neighbors = |&(x, y): &(usize, usize)| -> Vec<(usize, usize)> {
        let cur = match grid.get((x, y)) {
            Some(v) => v,
            None => return vec![],
        };

        grid.neighbours((x, y), false)
            .filter(|&neighbor| Some(cur) == grid.get(neighbor))
            .collect()
    };

    // an n-sided polygon will have n-corners- need i sizes for subtraction reasons
    let count_corners = |region: &Vec<(i32, i32)>| -> usize {
        let dir: [(i32, i32); 4] = [(1, 0), (0, 1), (0, -1), (-1, 0)];

        dir.iter().fold(0, |side_count, dir| {
            let mut sides = HashSet::new();
            // detect region boundary side
            for pos in region {
                let tmp = (pos.0 + dir.0, pos.1 + dir.1);
                if !region.contains(&tmp) {
                    sides.insert(tmp);
                }
            }

            // remove linear segments
            let mut remove: HashSet<(i32, i32)> = HashSet::new();
            for side in &sides {
                let mut tmp = (side.0 + dir.1, side.1 + dir.0);
                // travel linear segment
                while sides.contains(&tmp) {
                    remove.insert(tmp);
                    tmp = (tmp.0 + dir.1, tmp.1 + dir.0); // straight line
                }
            }

            side_count + sides.len() - remove.len()
        })
    };

    let mut found: HashSet<(usize, usize)> = HashSet::new();

    Some(grid.items().fold(0, |total_price, ((r, c), _)| {
        // skip if already in a region
        if found.contains(&(r, c)) {
            return total_price;
        }

        // isolate region
        let region: Vec<(i32, i32)> = bfs_reach((r, c), get_neighbors)
            .map(|coord| {
                found.insert(coord);
                (coord.0 as i32, coord.1 as i32)
            })
            .collect();

        let corners = count_corners(&region);
        total_price + region.len() * corners
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(140));
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(80));
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(1206));
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(236));
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(368));
    }
}
