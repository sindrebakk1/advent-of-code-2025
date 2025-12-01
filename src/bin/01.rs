use std::ops::Index;

advent_of_code::solution!(1);

pub struct Dial {
    numbers: [u8; 100],
    position: usize,
}

impl Dial {
    pub fn new() -> Dial {
        Dial { numbers: std::array::from_fn(|i| { i as u8 }), position: 50 }
    }
}

impl Index<usize> for Dial {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.numbers[index]
    }
}

impl Dial {
    pub fn step_right(&mut self) {
        let len = self.numbers.len();
        self.position = (self.position + 1) % len;
    }

    pub fn step_left(&mut self) {
        let len = self.numbers.len();
        self.position = (self.position + len - 1) % len;
    }

    pub fn turn_right<F>(&mut self, n: usize, mut on_tick: F)
    where F: FnMut(u8),
    {
        for _ in 0..n {
            self.step_right();
            on_tick(self.current());
        }
    }

    pub fn turn_left<F>(&mut self, n: usize, mut on_tick: F)
    where F: FnMut(u8),
    {
        for _ in 0..n {
            self.step_left();
            on_tick(self.current());
        }
    }

    pub fn current(&self) -> u8 {
        self.numbers[self.position]
    }
}

pub enum Direction {
    Left,
    Right,
}

pub fn parse_line(line: &str) -> Option<(Direction, usize)> {
    let (dir_str, num_str) = line.split_at(1);
    let dir = dir_str.chars().next()?;
    let steps: isize = num_str.parse().ok()?;

    match dir {
        'L' => Some((Direction::Left, steps as usize)),
        'R' => Some((Direction::Right, steps as usize)),
        _ => return None,
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut dial = Dial::new();
    input.lines().map(parse_line).try_fold(0u64, |mut acc, cur| {
        let (dir, steps) = cur?;
        match dir {
            Direction::Left => dial.turn_left(steps, |_| {}),
            Direction::Right => dial.turn_right(steps, |_| {}),
        };
        if dial.current() == 0 { acc += 1 };
        Some(acc)
    })
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut dial = Dial::new();
    input.lines().map(parse_line).try_fold(0u64, |mut acc, cur| {
        let (dir, steps) = cur?;
        match dir {
            Direction::Left => dial.turn_left(steps, |current| {
                if current == 0 {
                    acc += 1;
                }
            }),
            Direction::Right => dial.turn_right(steps, |current| {
                if current == 0 {
                    acc += 1;
                }
            })
        };
        Some(acc)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
