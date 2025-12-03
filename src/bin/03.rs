advent_of_code::solution!(3);

pub fn largest_jolt(bank: Vec<u8>, bank_size: usize) -> Option<u64> {
    let len = bank.len();
    let mut digits = Vec::with_capacity(bank_size);

    let slice = &bank[..len - bank_size + 1];
    let max = *slice.iter().max()?;
    let mut idx = slice.iter().position(|&x| x == max)?;

    digits.push(bank[idx]);

    for n in 1..bank_size {
        let slice = &bank[idx + 1..len - bank_size + 1 + n];
        let max = *slice.iter().max()?;
        idx += slice.iter().position(|&x| x == max)? + 1;
        digits.push(bank[idx]);
    }

    Some(digits.iter().fold(0, |acc, x| acc * 10 + *x as u64))
}

pub fn part_one(input: &str) -> Option<u64> {
    input.trim().lines().try_fold(0, |mut acc, line| {
        let bank = line
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect::<Vec<_>>();
        let largest = largest_jolt(bank, 2)?;
        acc += largest;
        Some(acc)
    })
}

pub fn part_two(input: &str) -> Option<u64> {
    input.trim().lines().try_fold(0, |mut acc, line| {
        let bank = line
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect::<Vec<_>>();
        let largest = largest_jolt(bank, 12)?;
        acc += largest;
        Some(acc)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
