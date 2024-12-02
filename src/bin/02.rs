

advent_of_code::solution!(2);

fn is_safe(sequence: &[u32]) -> bool {
    let is_ordered = sequence.windows(2).all(|pair| pair[0] < pair[1])
        || sequence.windows(2).all(|pair| pair[0] > pair[1]);
    let has_small_diffs = sequence
        .windows(2)
        .all(|pair| pair[0].abs_diff(pair[1]) <= 3);
    is_ordered && has_small_diffs
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut total_safe = 0;

    for line in input.lines() {
        let nums: Vec<u32> = line
            .split_ascii_whitespace()
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();

        if nums.len() < 2 {
            continue;
        }

        if is_safe(&nums) {
            total_safe += 1;
        }
    }

    Some(total_safe)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total_safe = 0;

    for line in input.lines() {
        let nums: Vec<u32> = line
            .split_ascii_whitespace()
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();

        if nums.len() < 2 {
            continue;
        }

        if is_safe(&nums) {
            total_safe += 1;
        } else {
            // remove 1 element at a time and check
            for i in 0..nums.len() {
                let mut modified = nums.clone();
                modified.remove(i);

                if is_safe(&modified) {
                    total_safe += 1;
                    break;
                }
            }
        }
    }

    Some(total_safe)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
