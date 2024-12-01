use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left_col, mut right_col): (Vec<u32>, Vec<u32>) = input
        .lines()
        .filter_map(|line| {
            let nums: Vec<u32> = line
                .split_ascii_whitespace()
                .filter_map(|s| s.parse::<u32>().ok())
                .collect();
            if nums.len() == 2 {
                Some((nums[0], nums[1]))
            } else {
                None
            }
        })
        .unzip();

    left_col.sort();
    right_col.sort();

    let total_diff: u32 = left_col
        .iter()
        .zip(&right_col)
        .map(|(left, right)| left.abs_diff(*right))
        .sum();

    Some(total_diff)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut left_col = Vec::new();
    let mut right_col = HashMap::new();

    input.lines().for_each(|line| {
        let nums: Vec<u32> = line
            .split_ascii_whitespace()
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();
        if nums.len() == 2 {
            left_col.push(nums[0]);
            *right_col.entry(nums[1]).or_insert(0) += 1;
        }
    });

    let total_diff: u32 = left_col
        .into_iter()
        .map(|value| value * right_col.get(&value).copied().unwrap_or(0))
        .sum();

    Some(total_diff)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
