use arrayvec::ArrayVec;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Write};
use std::hash::Hash;

advent_of_code::solution!(11);

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum DeviceId {
    Normal([u8; 3]),
    Output,
}

impl DeviceId {
    pub const YOU: Self = DeviceId::Normal([b'y', b'o', b'u']);
    pub const DAC: Self = DeviceId::Normal([b'd', b'a', b'c']);
    pub const FFT: Self = DeviceId::Normal([b'f', b'f', b't']);
    pub const SVR: Self = DeviceId::Normal([b's', b'v', b'r']);

    pub fn from_hash(hash: [u8; 3]) -> DeviceId {
        match hash {
            [b'o', b'u', b't'] => DeviceId::Output,
            [_, _, _] => DeviceId::Normal(hash),
        }
    }
}

impl Debug for DeviceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeviceId::Normal(hash) => {
                for &c in hash {
                    f.write_char(c as char)?;
                }
                Ok(())
            }
            DeviceId::Output => f.write_str("out"),
        }
    }
}

pub fn parse_input(input: &str) -> HashMap<DeviceId, ArrayVec<DeviceId, 32>> {
    let mut map = HashMap::new();

    input.trim().lines().for_each(|l| {
        let (id_str, outputs_str) = l.split_once(':').unwrap();

        let id_hash = id_str
            .trim()
            .chars()
            .map(|c| c as u8)
            .take(3)
            .collect_array::<3>()
            .unwrap();

        let id = DeviceId::from_hash(id_hash);

        let outputs: ArrayVec<DeviceId, 32> = outputs_str
            .trim()
            .split(' ')
            .map(|s| {
                s.trim()
                    .chars()
                    .map(|c| c as u8)
                    .take(3)
                    .collect_array::<3>()
                    .unwrap()
            })
            .map(DeviceId::from_hash)
            .collect();

        map.insert(id, outputs);
    });

    map
}

pub fn part_one(input: &str) -> Option<u64> {
    let map = parse_input(input);
    let mut num_paths = 0;
    let mut queue = VecDeque::from(vec![DeviceId::YOU]);

    while let Some(id) = queue.pop_front() {
        for &output in map.get(&id)? {
            if output == DeviceId::Output {
                num_paths += 1;
                break;
            }
            queue.push_back(output);
        }
    }

    Some(num_paths)
}

#[derive(Clone, Eq, PartialEq)]
pub struct PathStep {
    pub seen_dac: bool,
    pub seen_fft: bool,
    pub device_id: DeviceId,
    pub visited: HashSet<DeviceId>,
}

impl PathStep {
    pub fn new(device_id: DeviceId) -> Self {
        let mut visited = HashSet::new();
        visited.insert(device_id);
        PathStep {
            seen_dac: false,
            seen_fft: false,
            visited,
            device_id,
        }
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let map = parse_input(input);
    let mut num_paths = 0;
    let mut queue = VecDeque::from(vec![PathStep::new(DeviceId::SVR)]);

    while let Some(path_step) = queue.pop_front() {
        for &output in map.get(&path_step.device_id)? {
            if path_step.visited.contains(&output) {
                continue;
            }
            let mut next = path_step.clone();
            next.visited.insert(output);
            match output {
                DeviceId::FFT => {
                    next.seen_fft = true;
                }
                DeviceId::DAC => {
                    next.seen_dac = true;
                }
                DeviceId::Output => {
                    if next.seen_dac && next.seen_fft {
                        num_paths += 1;
                    }
                    continue;
                }
                _ => (),
            }
            next.device_id = output;
            queue.push_back(next);
        }
    }

    Some(num_paths)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2));
    }
}
