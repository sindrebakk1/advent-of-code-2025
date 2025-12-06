use std::cmp::{max, min};

advent_of_code::solution!(5);

pub fn parse_input(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let (ranges, ids) = input.trim().split_once("\n\n").unwrap();

    let ranges = ranges
        .trim()
        .lines()
        .map(|range| {
            let (start, end) = range.split_once('-').unwrap();
            (start.parse().unwrap(), end.parse().unwrap())
        })
        .collect();

    let ids = ids.trim().lines().map(|id| id.parse().unwrap()).collect();

    (ranges, ids)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (ranges, ids) = parse_input(input);

    ids.iter().try_fold(0u64, |mut acc, id| {
        if ranges
            .iter()
            .any(|(start, end)| (start..=end).contains(&id))
        {
            acc += 1;
        }
        Some(acc)
    })
}

pub fn ranges_overlap((s1, e1): &(u64, u64), (s2, e2): &(u64, u64)) -> bool {
    (s1 >= s2 && s1 <= e2)
        || (e1 >= s2 && e1 <= e2)
        || (s2 >= s1 && s2 <= e1)
        || (e2 >= s1 && e2 <= e1)
}

pub fn merge_ranges(ranges: &[(u64, u64)]) -> Vec<(u64, u64)> {
    let mut merged = Vec::new();

    for range in ranges.iter() {
        let mut overlaps: Vec<usize> = Vec::new();
        for (i, existing_range) in merged.iter().enumerate() {
            if ranges_overlap(range, existing_range) {
                overlaps.push(i);
            }
        }
        if overlaps.is_empty() {
            merged.push(*range);
            continue;
        }
        overlaps.sort_by(|a, b| b.cmp(a));
        let new_range = overlaps.iter().fold(*range, |mut acc, &i| {
            acc.0 = min(acc.0, merged[i].0);
            acc.1 = max(acc.1, merged[i].1);
            acc
        });
        for i in overlaps {
            merged.remove(i);
        }
        merged.push(new_range);
    }

    merged
}

pub fn part_two(input: &str) -> Option<u64> {
    let (ranges, _) = parse_input(input);
    let merged = merge_ranges(&ranges);
    merged.iter().try_fold(0, |mut acc, (start, end)| {
        acc += end - (start - 1);
        Some(acc)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let (ranges, ids) = parse_input(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(ranges.len(), 4);
        assert_eq!(ids.len(), 6);
    }

    #[test]
    fn test_merge_ranges() {
        let (ranges, _) = parse_input(&advent_of_code::template::read_file("examples", DAY));
        let merged = merge_ranges(&ranges);
        assert_eq!(merged, vec![(3, 5), (10, 20)]);
    }

    #[test]
    fn test_ranges_overlap() {
        assert!(!ranges_overlap(&(3, 5), &(10, 14)));
        assert!(ranges_overlap(&(16, 20), &(12, 18)));
        assert!(ranges_overlap(&(16, 20), &(17, 19)));
        assert!(ranges_overlap(&(16, 20), &(17, 22)));
        assert!(!ranges_overlap(&(10, 14), &(16, 20)));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
