use std::ops::Index;

advent_of_code::solution!(1);

// pub struct Dial{
//     pub index: usize,
//     pub wrap_count: u64,
//     pub numbers: [u8; 100],
// }
//
// impl Dial {
//     pub fn new() -> Self {
//         Self {
//             index: 50,
//             wrap_count: 0,
//             numbers: std::array::from_fn(|i| i as u8),
//         }
//     }
//
//     pub fn peek(&self) -> u8 {
//         self.numbers[self.index]
//     }
//
//     pub fn right(&mut self, steps: isize) {
//         self.update_index(steps);
//     }
//
//     pub fn left(&mut self, steps: isize) {
//         self.update_index(-steps);
//     }
//
//     fn update_index(&mut self, delta: isize) {
//         let len = self.numbers.len() as isize;
//
//         let old = self.index as isize;
//         let new = old + delta;
//
//         // detect wrap past zero
//         if delta > 0 && new >= len {
//             self.wrap_count += 1;
//         } else if delta < 0 && new < 0 {
//             self.wrap_count -= 1;
//         }
//
//         self.index = Self::wrap(new, len as usize);
//     }
//
//     #[inline]
//     fn wrap(pos: isize, len: usize) -> usize {
//         ((pos % len as isize) + len as isize) as usize % len
//     }
// }
//
// pub fn part_one(input: &str) -> Option<u64> {
//     let mut d = Dial::new();
//     let mut res: u64 = 0;
//
//     for line in input.lines().map(str::trim).filter(|l| !l.is_empty()) {
//         let (dir_str, num_str) = line.split_at(1);
//         let dir = dir_str.chars().next()?;
//         let steps: isize = num_str.parse().ok()?;
//
//         match dir {
//             'L' => d.left(steps),
//             'R' => d.right(steps),
//             _ => return None,
//         }
//         if d.peek() == 0 {
//             res += 1;
//         }
//     }
//     Some(res)
// }
//
// pub fn part_two(input: &str) -> Option<u64> {
//     let mut d = Dial::new();
//
//     for line in input.lines().map(str::trim).filter(|l| !l.is_empty()) {
//         // split into first char and the rest
//         let (dir_str, num_str) = line.split_at(1);
//         let dir = dir_str.chars().next()?;
//         let steps: isize = num_str.parse().ok()?;
//
//         match dir {
//             'L' => d.left(steps),
//             'R' => d.right(steps),
//             _ => return None,
//         };
//     }
//     Some(d.wrap_count)
// }

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
