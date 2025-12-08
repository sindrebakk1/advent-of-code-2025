use advent_of_code::{DSU, IVec3, OrdF64};
use std::collections::BinaryHeap;

advent_of_code::solution!(8);

pub fn parse_input(input: &str) -> Vec<IVec3> {
    input
        .lines()
        .map(|line| {
            let (x, rest) = line.split_once(',').unwrap();
            let (y, z) = rest.split_once(',').unwrap();

            let x = x.parse().unwrap();
            let y = y.parse().unwrap();
            let z = z.parse().unwrap();

            IVec3::new(x, y, z)
        })
        .collect()
}
pub fn take_sorted_pairs(boxes: &[IVec3], count: usize) -> Vec<(usize, usize)> {
    let mut heap: BinaryHeap<(OrdF64, (usize, usize))> = BinaryHeap::with_capacity(count);

    for i in 0..boxes.len() {
        for j in (i + 1)..boxes.len() {
            let d = boxes[i].distance(boxes[j]);
            let item = (OrdF64(d), (i, j));

            if heap.len() < count {
                heap.push(item);
            } else if item < *heap.peek().unwrap() {
                heap.pop();
                heap.push(item);
            }
        }
    }

    heap.into_sorted_vec()
        .into_iter()
        .map(|(_, ij)| ij)
        .collect()
}

pub fn count_circuits(input: &str, count: usize) -> Option<u64> {
    let junction_boxes = parse_input(input);

    let n = junction_boxes.len();

    let pairs = take_sorted_pairs(&junction_boxes, count);

    let mut dsu = DSU::new(n);
    for (i, j) in pairs {
        dsu.union(i, j);
    }

    let mut sizes = dsu.component_sizes();
    sizes.sort_unstable_by(|a, b| b.cmp(a)); // descending

    let result = sizes.iter().take(3).product::<u64>();
    Some(result)
}

pub fn part_one(input: &str) -> Option<u64> {
    count_circuits(input, 1000)
}

pub fn collect_sorted_pairs(boxes: &[IVec3]) -> Vec<(usize, usize)> {
    let n = boxes.len();
    if n < 2 {
        return Vec::new();
    }

    let num_pairs = n * (n - 1) / 2;
    let mut pairs: Vec<(OrdF64, (usize, usize))> = Vec::with_capacity(num_pairs);

    for i in 0..n {
        for j in (i + 1)..n {
            let d = boxes[i].distance(boxes[j]);
            pairs.push((OrdF64(d), (i, j)));
        }
    }

    pairs.sort_unstable_by(|(da, _), (db, _)| da.cmp(db)); // ascending distance

    pairs.into_iter().map(|(_, ij)| ij).collect()
}

pub fn part_two(input: &str) -> Option<u64> {
    let junction_boxes = parse_input(input);
    let n = junction_boxes.len();
    if n < 2 {
        return None;
    }

    let pairs = collect_sorted_pairs(&junction_boxes);
    let mut dsu = DSU::new(n);

    for (i, j) in pairs {
        dsu.union(i, j);

        if dsu.component_size(i) == n as u64 {
            let a = junction_boxes[i];
            let b = junction_boxes[j];
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
