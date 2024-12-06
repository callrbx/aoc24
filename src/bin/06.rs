advent_of_code::solution!(6);

use ndarray::Array2;
use rayon::prelude::*;
use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    // parse to grid, strip newlines
    let flat_grid: Vec<char> = input.chars().filter(|c| !c.is_whitespace()).collect();

    // grid is square
    let side_len = (flat_grid.len() as f64).sqrt() as usize;

    let grid = Array2::from_shape_vec((side_len, side_len), flat_grid).unwrap();

    let mut start_pos = None;
    let start_dir = '^';

    // find starting position, only starts ^ (for my case at least)
    for ((y, x), &value) in grid.indexed_iter() {
        if value == start_dir {
            start_pos = Some((x as isize, y as isize)); // (col, row)
            break;
        }
    }

    let start_pos = start_pos.unwrap();

    // series of right turns n -> w -> s -> e
    let moves = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut visited: HashSet<(isize, isize)> = HashSet::new();

    visited.insert(start_pos);

    let mut cur_pos = start_pos;
    let mut dir_idx = 0;

    loop {
        // find next spot
        let delta = moves[dir_idx];
        let next_pos = (cur_pos.0 + delta.0, cur_pos.1 + delta.1);

        // check if leaving areas
        if next_pos.0 < 0
            || next_pos.1 < 0
            || next_pos.0 >= side_len as isize
            || next_pos.1 >= side_len as isize
        {
            break;
        }

        // check obstacle
        if grid[[next_pos.1 as usize, next_pos.0 as usize]] == '#' {
            // turn right
            dir_idx = (dir_idx + 1) % 4;
        } else {
            // go forward
            cur_pos = next_pos;
            visited.insert(cur_pos);
        }
    }

    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    // parse to grid, strip newlines
    let flat_grid: Vec<char> = input.chars().filter(|c| !c.is_whitespace()).collect();

    // grid is square
    let side_len = (flat_grid.len() as f64).sqrt() as usize;

    let grid = Array2::from_shape_vec((side_len, side_len), flat_grid).unwrap();

    let mut start_pos = None;
    let start_dir = '^';

    // find starting position, only starts ^ (for my case at least)
    for ((y, x), &value) in grid.indexed_iter() {
        if value == start_dir {
            start_pos = Some((x as isize, y as isize)); // (col, row)
            break;
        }
    }

    let start_pos = start_pos.unwrap();

    // series of right turns n -> w -> s -> e
    let moves = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut visited: HashSet<(isize, isize)> = HashSet::new();

    visited.insert(start_pos);

    let mut cur_pos = start_pos;
    let mut dir_idx = 0;

    loop {
        // find next spot
        let delta = moves[dir_idx];
        let next_pos = (cur_pos.0 + delta.0, cur_pos.1 + delta.1);

        // check if leaving areas
        if next_pos.0 < 0
            || next_pos.1 < 0
            || next_pos.0 >= side_len as isize
            || next_pos.1 >= side_len as isize
        {
            break;
        }

        // check obstacle
        if grid[[next_pos.1 as usize, next_pos.0 as usize]] == '#' {
            // turn right
            dir_idx = (dir_idx + 1) % 4;
        } else {
            // go forward
            cur_pos = next_pos;
            visited.insert(cur_pos);
        }
    }

    // rayon parallel iter
    let obstruction_spots: u32 = visited
        .par_iter()
        .filter_map(|&pos| {
            // skip obstacles or start pos
            let obstruction = (pos.1 as usize, pos.0 as usize);
            if grid[[obstruction.0, obstruction.1]] == '#' || pos == start_pos {
                return None;
            }

            if simulate_guard(&grid, start_pos, &moves, side_len, obstruction) {
                Some(1)
            } else {
                None
            }
        })
        .sum();

    Some(obstruction_spots)
}

fn simulate_guard(
    grid: &Array2<char>,
    start_pos: (isize, isize),
    moves: &[(isize, isize)],
    side_len: usize,
    test_obstruction: (usize, usize),
) -> bool {
    let mut visited: HashSet<((isize, isize), usize)> = HashSet::new();
    let mut cur_pos = start_pos;
    let mut dir_idx = 0;

    loop {
        if visited.contains(&(cur_pos, dir_idx)) {
            return true; // loop detected
        }
        visited.insert((cur_pos, dir_idx));

        let delta = moves[dir_idx];
        let next_pos = (cur_pos.0 + delta.0, cur_pos.1 + delta.1);

        if next_pos.0 < 0
            || next_pos.1 < 0
            || next_pos.0 >= side_len as isize
            || next_pos.1 >= side_len as isize
        {
            return false; //off grid
        }

        if grid[[next_pos.1 as usize, next_pos.0 as usize]] == '#'
            || (next_pos.1 as usize, next_pos.0 as usize) == test_obstruction
        {
            dir_idx = (dir_idx + 1) % 4;
        } else {
            cur_pos = next_pos;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
