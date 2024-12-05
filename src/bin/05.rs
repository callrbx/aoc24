use std::collections::HashMap;

advent_of_code::solution!(5);

#[derive(Debug)]
struct Rule {
    page: u32,
    before: u32,
}

#[derive(Debug, Eq, Clone)]
struct Page {
    page_number: u32,
    order_rules: HashMap<u32, bool>, // pages this one comes
}

impl Page {
    fn new(page_number: u32, order_rules: HashMap<u32, bool>) -> Self {
        Self {
            page_number,
            order_rules,
        }
    }
}

impl PartialEq for Page {
    fn eq(&self, other: &Self) -> bool {
        self.page_number == other.page_number
    }
}

// shut up clippy, im doing bad things
#[allow(clippy::non_canonical_partial_ord_impl)]
impl PartialOrd for Page {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.order_rules.contains_key(&other.page_number) {
            // self comes before other
            Some(std::cmp::Ordering::Less)
        } else if other.order_rules.contains_key(&self.page_number) {
            // other comes before self
            Some(std::cmp::Ordering::Greater)
        } else {
            // no rule - not possible; pages unique(?)
            Some(std::cmp::Ordering::Equal)
        }
    }
}

impl Ord for Page {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
    }
}

fn is_valid_update(update_pages: &[u32], rules: &Vec<Rule>) -> bool {
    let positions: std::collections::HashMap<u32, usize> = update_pages
        .iter()
        .enumerate()
        .map(|(idx, &page)| (page, idx))
        .collect();

    for rule in rules {
        if let (Some(&pos_page), Some(&pos_before)) =
            (positions.get(&rule.page), positions.get(&rule.before))
        {
            if pos_page >= pos_before {
                return false;
            }
        }
    }

    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let (orders, instrs) = input.split_once("\n\n").unwrap();

    // parse the orders
    let rules: Vec<Rule> = orders
        .lines()
        .filter_map(|o| {
            o.split_once("|").map(|(num, before)| Rule {
                page: num.parse().unwrap(),
                before: before.parse().unwrap(),
            })
        })
        .collect();

    let sum_middle = instrs.lines().fold(0, |acc, update| {
        let pages: Vec<u32> = update.split(',').filter_map(|c| c.parse().ok()).collect();

        if is_valid_update(&pages, &rules) {
            acc + pages.get(pages.len() / 2).unwrap()
        } else {
            acc
        }
    });

    Some(sum_middle)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (orders, updates) = input.split_once("\n\n").unwrap();

    // (page_num, Vec<pages we need to be before>)
    let mut page_rules: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut rules: Vec<Rule> = Vec::new();

    for page in orders.lines() {
        if let Some((num, before)) = page.split_once('|') {
            let num: u32 = num.parse().unwrap();
            let before: u32 = before.parse().unwrap();
            page_rules.entry(num).or_default().push(before);
            rules.push(Rule { page: num, before });
        }
    }

    let sum_middle = updates.lines().fold(0, |acc, update| {
        let pages: Vec<u32> = update.split(',').filter_map(|c| c.parse().ok()).collect();

        let mut page_structs: Vec<Page> = pages
            .iter()
            .map(|&page_number| {
                let mut order_rules = HashMap::new();
                if let Some(after_pages) = page_rules.get(&page_number) {
                    for &after_page in after_pages {
                        order_rules.insert(after_page, true);
                    }
                }
                Page::new(page_number, order_rules)
            })
            .collect();

        // check if valid; if not, sort
        if is_valid_update(&pages, &rules) {
            acc
        } else {
            page_structs.sort();

            acc + page_structs
                .get(page_structs.len() / 2)
                .unwrap()
                .page_number
        }
    });

    Some(sum_middle)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
