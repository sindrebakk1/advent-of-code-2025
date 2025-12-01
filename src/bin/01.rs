advent_of_code::solution!(1);

pub struct Dial {
    numbers: [u8; 100],
    position: usize,
}

impl Default for Dial {
    fn default() -> Self {
        Dial {
            numbers: std::array::from_fn(|i| i as u8),
            position: 50,
        }
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

    pub fn step(&mut self, dir: Direction) {
        match dir {
            Direction::Left => self.step_right(),
            Direction::Right => self.step_left(),
        }
    }

    pub fn turn(&mut self, dir: Direction, steps: usize) {
        for _ in 0..steps {
            self.step(dir);
        }
    }

    pub fn turn_with<F>(&mut self, dir: Direction, steps: usize, mut on_tick: F)
    where
        F: FnMut(u8),
    {
        for _ in 0..steps {
            self.step(dir);
            on_tick(self.numbers[self.position]);
        }
    }

    pub fn current(&self) -> u8 {
        self.numbers[self.position]
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
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
        _ => None,
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut dial = Dial::default();
    input
        .lines()
        .map(parse_line)
        .try_fold(0, |mut acc, cur| {
            let (dir, steps) = cur?;
            dial.turn(dir, steps);
            if dial.current() == 0 {
                acc += 1
            };
            Some(acc)
        })
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut dial = Dial::default();
    input
        .lines()
        .map(parse_line)
        .try_fold(0, |mut acc, cur| {
            let (dir, steps) = cur?;
            dial.turn_with(dir, steps, |current| {
                if current == 0 {
                    acc += 1;
                }
            });
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
