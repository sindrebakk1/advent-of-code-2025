use arrayvec::ArrayVec;
use std::collections::HashSet;

advent_of_code::solution!(7);

pub fn parse_input(input: &str) -> (usize, Vec<Vec<bool>>) {
    let mut start_pos: usize = 0;

    let map = input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        start_pos = x;
                    }
                    c == '^'
                })
                .collect()
        })
        .collect();

    (start_pos, map)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (start_pos, map) = parse_input(input);

    let mut beams = HashSet::with_capacity(map[0].len());
    beams.insert(start_pos);

    let mut splits = 0;

    for row in map.iter() {
        let mut to_remove: ArrayVec<usize, 142> = ArrayVec::new();
        let mut to_insert: ArrayVec<usize, 142> = ArrayVec::new();
        for &beam in beams.iter() {
            if row[beam] {
                splits += 1;
                to_remove.push(beam);
                to_insert.push(beam - 1);
                to_insert.push(beam + 1);
            }
        }
        for old in to_remove {
            beams.remove(&old);
        }
        for new in to_insert {
            beams.insert(new);
        }
    }

    Some(splits)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (start, map) = parse_input(input);

    let mut beams = [0u64; 142];
    beams[start] = 1;

    let mut next = [0u64; 142];
    let mut timelines: u64 = 1;

    for row in map.iter() {
        next.fill(0);

        for i in 0..142 {
            let count = beams[i];
            if count == 0 {
                continue;
            }

            if row[i] {
                timelines += count;
                next[i - 1] += count;
                next[i + 1] += count;
            } else {
                next[i] += count;
            }
        }

        beams = next;
    }

    Some(timelines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
