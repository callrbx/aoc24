advent_of_code::solution!(7);

use itertools::Itertools;
use rayon::prelude::*;

#[derive(Debug)]
struct Equation {
    test_value: usize,
    ops: Vec<usize>,
}

impl Equation {
    fn new(input: &str) -> Option<Self> {
        let (ans, remaining) = input.split_once(": ").unwrap();
        let ans: usize = match ans.parse() {
            Ok(ans) => ans,
            Err(_) => {
                eprintln!("Bad Line: {}", input);
                return None;
            }
        };
        let ops: Vec<usize> = remaining
            .split(" ")
            .map(|op| op.parse().unwrap())
            .collect();
        Some(Self {
            test_value: ans,
            ops,
        })
    }

    // return our answer if valid, otherwise 0
    fn is_valid(&self, part2: bool) -> usize {
        let n = self.ops.len();
        if n < 2 {
            return 0; // cant eval 2 numbers
        }

        // generate all possible combinations of ops with +, *, | (concat, keeping it single char)
        let op_combos = if part2 {
            (0..n - 1)
                .map(|_| vec!['+', '*', '|'])
                .multi_cartesian_product()
        } else {
            (0..n - 1).map(|_| vec!['+', '*']).multi_cartesian_product()
        };

        for ops in op_combos {
            if self.eval(&ops) == self.test_value {
                return self.test_value;
            }
        }

        0
    }

    // do math on statement with the ops vector
    fn eval(&self, ops: &[char]) -> usize {
        let mut result = self.ops[0];
        for (i, &op) in ops.iter().enumerate() {
            match op {
                '+' => result += self.ops[i + 1],
                '*' => result *= self.ops[i + 1],
                '|' => {
                    let concat = format!("{}{}", result, self.ops[i + 1])
                        .parse::<usize>()
                        .unwrap();
                    result = concat;
                }
                _ => panic!("bad op"),
            }
            // early exit
            if result > self.test_value {
                return 0;
            }
        }
        result
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    // process lines in to equations
    let equs: Vec<Equation> = input
        .lines()
        .filter_map(Equation::new)
        .collect();

    // parallel iterate to check if valid
    Some(equs.par_iter().map(|equ| equ.is_valid(false)).sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    // process lines in to equations
    let equs: Vec<Equation> = input
        .lines()
        .filter_map(Equation::new)
        .collect();

    // parallel iterate to check if valid
    Some(equs.par_iter().map(|equ| equ.is_valid(true)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
