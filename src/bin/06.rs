use itertools::Itertools;
use std::cmp::max;

advent_of_code::solution!(6);

#[derive(Copy, Clone, Debug)]
pub enum Operation {
    Add,
    Multiply,
}

fn apply_op(a: u64, b: u64, op: Operation) -> u64 {
    match op {
        Operation::Add => a + b,
        Operation::Multiply => a * b,
    }
}

pub fn parse_input_p1(input: &str) -> Vec<(Operation, Vec<u64>)> {
    let rows = input
        .trim()
        .lines()
        .rev()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let len = rows.len();
    let mut problems: Vec<(Operation, Vec<u64>)> = rows[0]
        .iter()
        .map(|&c| {
            let op = match c {
                "+" => Operation::Add,
                "*" => Operation::Multiply,
                _ => panic!("invalid input"),
            };
            (op, Vec::with_capacity(len - 1))
        })
        .collect();

    for (i, (_, values)) in problems.iter_mut().enumerate() {
        for row in rows.iter().skip(1) {
            values.push(row[i].parse().unwrap());
        }
    }

    problems
}

pub fn part_one(input: &str) -> Option<u64> {
    parse_input_p1(input)
        .iter()
        .try_fold(0, |mut acc, (op, values)| {
            let res = values
                .iter()
                .skip(1)
                .fold(values[0], |a, &b| apply_op(a, b, *op));
            acc += res;
            Some(acc)
        })
}

pub fn parse_input_p2(input: &str) -> Vec<(Operation, Vec<u64>)> {
    let rows = input
        .trim()
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let max_len = rows.iter().fold(0, |acc, row| max(acc, row.len()));

    let rows = rows
        .iter()
        .map(|row| {
            row.iter()
                .copied()
                .pad_using(max_len, |_| ' ')
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let len = rows.len();

    let operators = &rows[len - 1];

    let problems: Vec<(Operation, Vec<u64>)> = operators
        .iter()
        .chain(vec![&' ', &'$'])
        .positions(|&c| c == '+' || c == '*' || c == '$')
        .tuple_windows()
        .map(|(start, end)| {
            let op = match operators[start] {
                '+' => Operation::Add,
                '*' => Operation::Multiply,
                _ => panic!("invalid input"),
            };

            let mut values = Vec::with_capacity((end - 1) - start);

            for x in (start..end - 1).rev() {
                let mut chars: Vec<char> = Vec::with_capacity(len - 1);
                for row in rows.iter().take(len - 1) {
                    chars.push(row[x]);
                }

                let value = chars
                    .into_iter()
                    .collect::<String>()
                    .trim()
                    .parse()
                    .unwrap();

                values.push(value);
            }

            (op, values)
        })
        .collect();

    problems
}

pub fn part_two(input: &str) -> Option<u64> {
    parse_input_p2(input)
        .iter()
        .try_fold(0, |mut acc, (op, values)| {
            let res = values
                .iter()
                .skip(1)
                .fold(values[0], |a, &b| apply_op(a, b, *op));
            acc += res;
            Some(acc)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
