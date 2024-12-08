advent_of_code::solution!(8);

use itertools::Itertools;
use std::collections::{HashMap, HashSet};

use ndarray::Array2;

#[allow(unused)]
fn print_map(
    grid: &ndarray::ArrayBase<ndarray::OwnedRepr<char>, ndarray::Dim<[usize; 2]>>,
    antinodes: &HashSet<(usize, usize)>,
) {
    for (coord, c) in grid.indexed_iter() {
        if antinodes.get(&coord).is_some() && c == &'.' {
            print!("#")
        } else {
            print!("{}", c)
        }
        if coord.1 == grid.ncols() - 1 {
            println!()
        }
    }
}

fn get_distance_antinode(a: &(usize, usize), b: &(usize, usize)) -> Vec<(usize, usize)> {
    // vector diff between antenna
    let diff_row = b.0 as isize - a.0 as isize;
    let diff_col = b.1 as isize - a.1 as isize;

    // calc antinodes
    let x1 = (a.0 as isize - diff_row, a.1 as isize - diff_col);
    let x2 = (b.0 as isize + diff_row, b.1 as isize + diff_col);

    vec![
        (x1.0 as usize, x1.1 as usize),
        (x2.0 as usize, x2.1 as usize),
    ]
}

pub fn part_one(input: &str) -> Option<usize> {
    let flat_grid: Vec<char> = input.chars().filter(|c| !c.is_ascii_whitespace()).collect();
    let side_len = (flat_grid.len() as f64).sqrt() as usize;

    let grid: ndarray::ArrayBase<ndarray::OwnedRepr<char>, ndarray::Dim<[usize; 2]>> =
        Array2::from_shape_vec((side_len, side_len), flat_grid).unwrap();

    let mut freqs: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for ((row, col), c) in grid.indexed_iter() {
        if *c != '.' {
            freqs.entry(*c).or_default().push((row, col));
        }
    }

    let mut antinodes_set: HashSet<(usize, usize)> = HashSet::new();

    for (_, antennas) in freqs {
        for (a, b) in antennas.iter().tuple_combinations() {
            let antinodes = get_distance_antinode(a, b);
            for antinode in antinodes {
                // use this for ongrid validation - none if oob
                if grid.get(antinode).is_some() {
                    antinodes_set.insert(antinode);
                }
            }
        }
    }

    // print_map(&grid, &antinodes_set);

    Some(antinodes_set.len())
}

fn get_diagonal_antinodes(
    a: &(usize, usize),
    b: &(usize, usize),
    grid_size: usize,
) -> Vec<(usize, usize)> {
    let mut points = Vec::new();

    fn gcd(mut a: isize, mut b: isize) -> isize {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }

    // https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
    let diff_row = b.0 as isize - a.0 as isize;
    let diff_col = b.1 as isize - a.1 as isize;
    let gcd = gcd(diff_row.abs(), diff_col.abs());

    let step_row = diff_row / gcd;
    let step_col = diff_col / gcd;

    // dont get full line only get outer diagonal line <- A - B ->
    // extend outward in both directions until we go off grid
    let mut cur = (a.0 as isize, a.1 as isize);
    while cur.0 >= 0 && cur.1 >= 0 && cur.0 < grid_size as isize && cur.1 < grid_size as isize {
        points.push((cur.0 as usize, cur.1 as usize));
        cur.0 -= step_row;
        cur.1 -= step_col;
    }

    cur = (b.0 as isize, b.1 as isize);
    while cur.0 >= 0 && cur.1 >= 0 && cur.0 < grid_size as isize && cur.1 < grid_size as isize {
        points.push((cur.0 as usize, cur.1 as usize));
        cur.0 += step_row;
        cur.1 += step_col;
    }

    points
}

pub fn part_two(input: &str) -> Option<usize> {
    let flat_grid: Vec<char> = input.chars().filter(|c| !c.is_ascii_whitespace()).collect();
    let side_len = (flat_grid.len() as f64).sqrt() as usize;

    let grid = Array2::from_shape_vec((side_len, side_len), flat_grid).unwrap();

    let mut freqs: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for ((row, col), c) in grid.indexed_iter() {
        if *c != '.' {
            freqs.entry(*c).or_default().push((row, col));
        }
    }

    let mut antinodes_set: HashSet<(usize, usize)> = HashSet::new();

    for (_, antennas) in freqs {
        for (a, b) in antennas.iter().tuple_combinations() {
            let antinodes = get_diagonal_antinodes(a, b, side_len);
            for antinode in antinodes {
                antinodes_set.insert(antinode);
            }
        }
    }

    // print_map(&grid, &antinodes_set);

    Some(antinodes_set.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2));
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(4));
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
