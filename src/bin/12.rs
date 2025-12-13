use itertools::{Either, Itertools};
use regex::Regex;
use std::fmt::{Debug, Write};
use std::io::Lines;

advent_of_code::solution!(12);

pub const PRESENT_SIZE: usize = 3;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct Present(pub [[bool; PRESENT_SIZE]; PRESENT_SIZE]);

impl Present {
    pub fn rotate_cw(&mut self) {
        let original = self.0;
        for i in 0..PRESENT_SIZE {
            for j in 0..PRESENT_SIZE {
                self.0[i][j] = original[PRESENT_SIZE - j - 1][i];
            }
        }
    }

    pub fn rotate_ccw(&mut self) {
        let original = self.0;
        for i in 0..PRESENT_SIZE {
            for j in 0..PRESENT_SIZE {
                self.0[i][j] = original[j][PRESENT_SIZE - i - 1];
            }
        }
    }
}

impl Debug for Present {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.0 {
            for cell in row {
                let c = match cell {
                    true => "# ",
                    false => ". ",
                };
                f.write_str(c)?;
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Region([[Option<bool>; 50]; 50]);

impl Region {
    pub fn new(w: usize, h: usize) -> Self {
        let mut grid = [[None; 50]; 50];
        for row in grid.iter_mut().take(h) {
            for cell in row.iter_mut().take(w) {
                *cell = Some(false);
            }
        }
        Region(grid)
    }

    pub fn place_present(&mut self, x: usize, y: usize, present: Present) -> bool {
        for i in 0..PRESENT_SIZE {
            for j in 0..PRESENT_SIZE {
                if present.0[i][j] {
                    let Some(blocked) = self.0[i + y][j + x] else {
                        return false;
                    };
                    if blocked {
                        return false;
                    }
                }
            }
        }

        for i in 0..PRESENT_SIZE {
            for j in 0..PRESENT_SIZE {
                if present.0[i][j] {
                    self.0[i + y][j + x] = Some(true);
                }
            }
        }
        true
    }
}

impl Debug for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.0 {
            let mut wrote = false;
            for cell in row {
                let _ = match cell {
                    None => break,
                    Some(true) => f.write_str("# "),
                    Some(false) => f.write_str(". "),
                };
                wrote = true;
            }
            if wrote {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

pub fn parse_input(input: &str) -> ([Present; 6], Vec<(Region, [u32; 6])>) {
    let mut lines = input.trim().lines().into_iter();
    let re = Regex::new(r"\d+x\d+:").unwrap();
    let (presents, regions): (Vec<&str>, Vec<&str>) =
        lines.partition_map(|line| match re.is_match(line) {
            true => Either::Right(line),
            false => Either::Left(line),
        });

    let presents = presents
        .chunks(5)
        .map(|chunk| {
            let present_shape = chunk
                .iter()
                .skip(1)
                .take(PRESENT_SIZE)
                .copied()
                .map(|l| {
                    l.chars()
                        .take(PRESENT_SIZE)
                        .map(|c| c == '#')
                        .collect_array::<3>()
                        .unwrap()
                })
                .collect_array::<3>()
                .unwrap();

            Present(present_shape)
        })
        .collect_array::<6>()
        .unwrap();

    let regions = regions
        .iter()
        .map(|line| {
            let (region_size_str, present_counts_str) = line.split_once(':').unwrap();
            let (size_x, size_y) = region_size_str.split_once('x').unwrap();
            let region = Region::new(size_x.parse().unwrap(), size_y.parse().unwrap());
            let present_counts = present_counts_str.split_whitespace().take(6).map(|count| count.parse().unwrap()).collect_array::<6>().unwrap();
            (region, present_counts)
        })
        .collect();
    (presents, regions)
}

pub fn part_one(input: &str) -> Option<u64> {
    None
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_present() {
        let mut present = Present([[true, true, true], [true, true, false], [true, true, false]]);

        present.rotate_cw();
        assert_eq!(
            present,
            Present([[true, true, true], [true, true, true], [false, false, true],])
        );
        print!("{present:?}");
        present.rotate_cw();
        assert_eq!(
            present,
            Present([[false, true, true], [false, true, true], [true, true, true],])
        );
        print!("{present:?}");
        present.rotate_cw();
        assert_eq!(
            present,
            Present([[true, false, false], [true, true, true], [true, true, true],])
        );
        print!("{present:?}");
        present.rotate_ccw();
        assert_eq!(
            present,
            Present([[false, true, true], [false, true, true], [true, true, true],])
        );
        print!("{present:?}");
        present.rotate_ccw();
        assert_eq!(
            present,
            Present([[true, true, true], [true, true, true], [false, false, true],])
        );
        print!("{present:?}");
        // assert!(false);
    }

    #[test]
    fn test_region() {
        let mut region = Region::new(10, 10);
        println!("{region:?}");

        let mut present = Present([[true, true, true], [true, true, false], [true, true, false]]);

        assert!(region.place_present(0, 0, present));
        println!("{region:?}");

        present.rotate_cw();

        assert!(region.place_present(3, 0, present));
        println!("{region:?}");

        present.rotate_cw();
        present.rotate_cw();

        assert!(region.place_present(2, 1, present));
        println!("{region:?}");

        assert!(!region.place_present(0, 2, present));
        println!("{region:?}");
        // assert!(false);
    }

    #[test]
    fn test_parse_input() {
        let (presents, regions) = parse_input(&advent_of_code::template::read_file("examples", DAY));
        for present in presents {
            println!("{present:?}");
        }
        for (region, presents) in regions {
            println!("{region:?}");
            println!("{presents:?}");
        }
        assert!(false);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
