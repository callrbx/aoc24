advent_of_code::solution!(7);

use rayon::prelude::*;

type Operator = fn(usize, usize) -> usize;

fn operator_add(a: usize, b: usize) -> usize {
    a + b
}

fn operator_concatenate(a: usize, b: usize) -> usize {
    let digits: usize = (b as f64).log10().floor() as usize + 1;
    a * 10usize.pow(digits as u32) + b
}

fn operator_multiply(a: usize, b: usize) -> usize {
    a * b
}

#[derive(Debug)]
struct Equation {
    test_value: usize,
    ops: Vec<usize>,
    solutions: usize,
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
        let ops: Vec<usize> = remaining.split(" ").map(|op| op.parse().unwrap()).collect();
        Some(Self {
            solutions: 0,
            test_value: ans,
            ops,
        })
    }

    fn is_valid(&mut self, part2: bool) -> usize {
        let operators: Vec<Operator> = if part2 {
            vec![operator_add, operator_multiply, operator_concatenate]
        } else {
            vec![operator_add, operator_multiply]
        };

        self.solve(&operators, self.ops[0], 1);

        if self.solutions > 0 {
            self.test_value
        } else {
            0
        }
    }

    fn solve(&mut self, operators: &[Operator], current_result: usize, index: usize) {
        if current_result > self.test_value {
            return;
        }

        if index == self.ops.len() {
            if current_result == self.test_value {
                self.solutions += 1;
            }
            return;
        }

        operators.iter().for_each(|&operator| {
            self.solve(
                operators,
                operator(current_result, self.ops[index]),
                index + 1,
            );
        });
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    // process lines in to equations
    let equs: Vec<Equation> = input.lines().filter_map(Equation::new).collect();

    // parallel iterate to check if valid
    Some(
        equs.into_par_iter()
            .fold(|| 0_usize, |acc, mut equ| equ.is_valid(false) + acc)
            .sum::<usize>(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    // process lines in to equations
    let equs: Vec<Equation> = input.lines().filter_map(Equation::new).collect();

    // parallel iterate to check if valid
    Some(
        equs.into_par_iter()
            .fold(|| 0_usize, |acc, mut equ| equ.is_valid(true) + acc)
            .sum::<usize>(),
    )
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
