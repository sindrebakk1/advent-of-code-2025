use advent_of_code::IVec3;
use itertools::Itertools;
use std::collections::HashSet;

advent_of_code::solution!(8);

pub fn parse_input(input: &str) -> Vec<IVec3> {
    input
        .lines()
        .map(|line| {
            let (x, y, z) = line
                .split(",")
                .map(|x| x.parse().unwrap())
                .collect_tuple()
                .unwrap();
            IVec3::new(x, y, z)
        })
        .collect()
}

pub fn take_sorted_pairs(boxes: &[IVec3], count: usize) -> Vec<(IVec3, IVec3)> {
    boxes
        .iter()
        .tuple_combinations()
        .map(|(&a, &b)| (a.distance(b), (a, b)))
        .sorted_by(|(a, _), (b, _)| a.total_cmp(b))
        .take(count)
        .map(|(_, b)| b)
        .collect()
}

pub fn count_circuits(input: &str, count: usize) -> Option<u64> {
    let junction_boxes = parse_input(input);
    let pairs = take_sorted_pairs(&junction_boxes, count);
    let mut circuits: Vec<HashSet<IVec3>> = Vec::with_capacity(pairs.len());

    for (a, b) in pairs {
        let matching_circuits: Vec<usize> = circuits
            .iter()
            .enumerate()
            .filter(|(_, c)| c.contains(&a) || c.contains(&b))
            .map(|(i, _)| i)
            .collect();
        if matching_circuits.is_empty() {
            circuits.push([a, b].iter().copied().collect());
            continue;
        }
        let first_match = *matching_circuits.first().unwrap();
        circuits[first_match].insert(a);
        circuits[first_match].insert(b);
        let mut rest = matching_circuits[1..].to_vec();
        while let Some(idx) = rest.pop() {
            let removed = circuits.remove(idx);
            for junction_box in removed {
                circuits[first_match].insert(junction_box);
            }
        }
    }

    let counts: Vec<u64> = circuits
        .iter()
        .map(|c| c.len() as u64)
        .sorted_by(|a, b| b.cmp(a))
        .take(3)
        .collect();

    Some(counts.iter().product::<u64>())
}

pub fn part_one(input: &str) -> Option<u64> {
    count_circuits(input, 1000)
}

pub fn collect_sorted_pairs(boxes: &[IVec3]) -> Vec<(IVec3, IVec3)> {
    boxes
        .iter()
        .tuple_combinations()
        .map(|(&a, &b)| (a.distance(b), (a, b)))
        .sorted_by(|(a, _), (b, _)| a.total_cmp(b))
        .map(|(_, b)| b)
        .collect()
}

pub fn part_two(input: &str) -> Option<u64> {
    let junction_boxes = parse_input(input);
    let pairs = collect_sorted_pairs(&junction_boxes);
    let mut circuits: Vec<HashSet<IVec3>> = Vec::with_capacity(pairs.len());

    for (a, b) in pairs {
        let matching_circuits: Vec<usize> = circuits
            .iter()
            .enumerate()
            .filter(|(_, c)| c.contains(&a) || c.contains(&b))
            .map(|(i, _)| i)
            .collect();
        if matching_circuits.is_empty() {
            circuits.push([a, b].iter().copied().collect());
            continue;
        }
        let first_match = *matching_circuits.first().unwrap();
        circuits[first_match].insert(a);
        circuits[first_match].insert(b);
        let mut rest = matching_circuits[1..].to_vec();
        while let Some(idx) = rest.pop() {
            let removed = circuits.remove(idx);
            for junction_box in removed {
                circuits[first_match].insert(junction_box);
            }
        }
        if circuits.iter().any(|c| c.len() == junction_boxes.len()) {
            return Some(a.x as u64 * b.x as u64);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = count_circuits(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
