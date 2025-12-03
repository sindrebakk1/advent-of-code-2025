use itertools::Itertools;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    fn is_invalid(id: u64) -> bool {
        let len = (id as f64).log10().floor() as u32 + 1;
        if !len.is_multiple_of(2) {
            return false;
        }
        let divisor = 10u64.pow(len / 2);
        (id / divisor) == (id % divisor)
    }

    input.trim().split(',').try_fold(0, |mut acc, range| {
        let (start, end) = range.split('-').collect_tuple()?;
        for id in start.parse::<u64>().ok()?..=end.parse::<u64>().ok()? {
            if is_invalid(id) {
                acc += id;
            }
        }
        Some(acc)
    })
}

pub fn part_two(input: &str) -> Option<u64> {
    fn is_invalid(id: u64) -> bool {
        let len = (id as f64).log10().floor() as u32 + 1;
        'outer: for pat_len in 1..=len / 2 {
            if !len.is_multiple_of(pat_len) {
                continue;
            }
            let divisor = 10u64.pow(pat_len);
            let pat = id % divisor;
            let mut rest = id;
            for _ in 0..len / pat_len {
                if rest % divisor != pat {
                    continue 'outer;
                }
                rest /= divisor;
            }
            return true;
        }
        false
    }

    input.trim().split(',').try_fold(0, |mut acc, range| {
        let (start, end) = range.split('-').collect_tuple()?;
        for id in start.parse::<u64>().ok()?..=end.parse::<u64>().ok()? {
            if is_invalid(id) {
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
