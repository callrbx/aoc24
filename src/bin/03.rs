advent_of_code::solution!(3);

use itertools::Itertools;
use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    // use named capture groups X and Y
    let re = Regex::new(r"mul\((?P<X>\d{1,3}),(?P<Y>\d{1,3})\)").unwrap();

    let sum: u32 = re.captures_iter(input).fold(0, |acc, cap| {
        let x = cap["X"].parse::<u32>().unwrap_or(0);
        let y = cap["Y"].parse::<u32>().unwrap_or(0);
        acc + (x * y)
    });

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re_mul = Regex::new(r"mul\((?P<X>\d{1,3}),(?P<Y>\d{1,3})\)").unwrap();
    let re_op = Regex::new(r"do\(\)|don't\(\)").unwrap();

    let mut do_mul = true;
    let mut total_sum: u32 = 0;

    let combined_matches = re_op
        .find_iter(input)
        .chain(re_mul.find_iter(input))
        .sorted_by_key(|m| m.start());

    for m in combined_matches {
        match m.as_str() {
            "do()" => do_mul = true,
            "don't()" => do_mul = false,
            _ => {
                if let Some(caps) = re_mul.captures(m.as_str()) {
                    if do_mul {
                        let x = caps["X"].parse::<u32>().unwrap_or(0);
                        let y = caps["Y"].parse::<u32>().unwrap_or(0);
                        total_sum += x * y;
                    }
                }
            }
        }
    }

    Some(total_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
