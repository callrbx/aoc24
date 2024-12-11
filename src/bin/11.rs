use cached::proc_macro::cached;

advent_of_code::solution!(11);

#[cached]
fn compute_stone(stone: usize, depth: usize) -> usize {
    let num_digits = (stone as f64).log10().floor() as usize + 1;

    if depth == 0 {
        return 1;
    }

    if stone == 0 {
        compute_stone(1, depth - 1)
    } else if num_digits % 2 == 0 {
        let half_len = num_digits / 2;
        let divisor = 10usize.pow(half_len as u32);

        let left_num = stone / divisor;
        let right_num = stone % divisor;

        compute_stone(left_num, depth - 1) + compute_stone(right_num, depth - 1)
    } else {
        compute_stone(stone * 2024, depth - 1)
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .split_ascii_whitespace()
            .filter_map(|sval| sval.parse::<usize>().ok())
            .map(|stone| compute_stone(stone, 25))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .split_ascii_whitespace()
            .filter_map(|sval| sval.parse::<usize>().ok())
            .map(|stone| compute_stone(stone, 75))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
