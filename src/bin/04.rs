advent_of_code::solution!(4);

use ndarray::Array2;

pub fn part_one(input: &str) -> Option<u32> {
    // parse to grid
    let flat_grid: Vec<char> = input.chars().filter(|c| c.is_alphabetic()).collect();

    // grid is square
    let side_len = (flat_grid.len() as f64).sqrt() as usize;

    let grid = Array2::from_shape_vec((side_len, side_len), flat_grid).unwrap();

    let word = "XMAS";
    let word_len = word.len();
    let (rows, cols) = grid.dim();
    let word_chars: Vec<char> = word.chars().collect();
    let mut count = 0;

    let directions = [
        (0, 1),   // e
        (0, -1),  // w
        (1, 0),   // s
        (-1, 0),  // n
        (1, 1),   // se
        (1, -1),  // sw
        (-1, 1),  // ne
        (-1, -1), // nw
    ];

    // would have been a filter map; closure borrow errors are dumb
    for r in 0..rows {
        for c in 0..cols {
            for &(dr, dc) in &directions {
                let mut found = true;
                for (i, cur_char) in word_chars.iter().enumerate().take(word_len) {
                    let nr = r as isize + dr * i as isize;
                    let nc = c as isize + dc * i as isize;
                    if nr < 0 || nc < 0 || nr >= rows as isize || nc >= cols as isize {
                        found = false;
                        break;
                    }
                    if grid[[nr as usize, nc as usize]] != *cur_char {
                        found = false;
                        break;
                    }
                }
                if found {
                    count += 1;
                }
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    // parse to grid
    let flat_grid: Vec<char> = input.chars().filter(|c| c.is_alphabetic()).collect();

    // grid is square
    let side_len = (flat_grid.len() as f64).sqrt() as usize;

    let grid = Array2::from_shape_vec((side_len, side_len), flat_grid).unwrap();

    // only check the diags; will check each in both directions
    let diagonals = [
        (-1, -1), // nw -> se
        (-1, 1),  // ne -> sw
    ];

    let count = (1..grid.nrows() - 1)
        .flat_map(|r| (1..grid.ncols() - 1).map(move |c| (r, c)))
        .filter(|&(r, c)| grid[[r, c]] == 'A') // center must be A
        .filter(|&(r, c)| {
            // collect and validate diag chars
            diagonals.iter().all(|&(dr, dc)| {
                let chars: Vec<_> = (0..3)
                    .map(|i| {
                        let row = (r as isize + dr * (i - 1)) as usize;
                        let col = (c as isize + dc * (i - 1)) as usize;
                        grid[[row, col]]
                    })
                    .collect();
                chars == ['M', 'A', 'S'] || chars == ['S', 'A', 'M']
            })
        })
        .count();

    Some(count as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
