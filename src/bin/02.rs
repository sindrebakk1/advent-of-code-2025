use itertools::Itertools;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    fn is_valid(id: String) -> bool {
        if !id.len().is_multiple_of(2) {
            return true;
        }
        let (p1, p2) = id.split_at(id.len() / 2);
        p1 != p2
    }

    input
        .trim()
        .split(',')
        .filter(|r| !r.is_empty())
        .try_fold(0, |mut acc, range| {
            let (start, end) = range.split('-').collect_tuple()?;
            for id in start.parse::<u64>().ok()?..=end.parse::<u64>().ok()? {
                if !is_valid(id.to_string()) {
                    acc += id;
                }
            }
            Some(acc)
        })
}

pub fn part_two(input: &str) -> Option<u64> {
    fn is_valid(id: String) -> bool {
        let bytes = id.as_bytes();
        let n = bytes.len();

        for pat_len in 1..=n / 2 {
            if !n.is_multiple_of(pat_len) {
                continue;
            }
            let pat = &bytes[..pat_len];
            if bytes.chunks(pat_len).all(|chunk| chunk == pat) {
                return false;
            }
        }
        true
    }

    input
        .trim()
        .split(',')
        .filter(|r| !r.is_empty())
        .try_fold(0, |mut acc, range| {
            let (start, end) = range.split('-').collect_tuple()?;
            for id in start.parse::<u64>().ok()?..=end.parse::<u64>().ok()? {
                if !is_valid(id.to_string()) {
                    acc += id;
                }
            }
            Some(acc)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
