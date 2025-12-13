use itertools::{Either, Itertools};
use regex::Regex;
use std::collections::VecDeque;
use std::fmt::Debug;

advent_of_code::solution!(12);

pub const PRESENT_SIZE: usize = 3;
pub const PRESENT_COUNT: usize = 6;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct Present(pub [[bool; PRESENT_SIZE]; PRESENT_SIZE]);

#[derive(Copy, Clone)]
pub enum Orientation {
    North,
    South,
    East,
    West,
}

impl Orientation {
    pub const ALL: [Orientation; 4] = [
        Orientation::North,
        Orientation::South,
        Orientation::East,
        Orientation::West,
    ];
}

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

    #[inline(always)]
    pub fn rotate(&self, orientation: Orientation) -> Present {
        let mut out = [[false; PRESENT_SIZE]; PRESENT_SIZE];

        for i in 0..PRESENT_SIZE {
            for j in 0..PRESENT_SIZE {
                out[i][j] = match orientation {
                    Orientation::North => self.0[i][j],
                    Orientation::East => self.0[PRESENT_SIZE - j - 1][i],
                    Orientation::South => self.0[PRESENT_SIZE - i - 1][PRESENT_SIZE - j - 1],
                    Orientation::West => self.0[j][PRESENT_SIZE - i - 1],
                };
            }
        }

        Present(out)
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
pub struct Region([[Option<bool>; 50]; 50], (u8, u8));

impl Region {
    pub fn new(w: usize, h: usize) -> Self {
        let mut grid = [[None; 50]; 50];
        for row in grid.iter_mut().take(h) {
            for cell in row.iter_mut().take(w) {
                *cell = Some(false);
            }
        }
        Region(grid, (w as u8, h as u8))
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

    pub fn place_present_at(&mut self, x: usize, y: usize, present: &Present, value: bool) {
        for i in 0..PRESENT_SIZE {
            for j in 0..PRESENT_SIZE {
                if present.0[i][j] {
                    self.0[i + y][j + x] = Some(value);
                }
            }
        }
    }

    pub fn width(&self) -> usize {
        self.1.0 as usize
    }

    pub fn height(&self) -> usize {
        self.1.1 as usize
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

pub fn parse_input(input: &str) -> ([Present; PRESENT_COUNT], Vec<(Region, [u8; PRESENT_COUNT])>) {
    let lines = input.trim().lines();
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
                        .collect_array::<PRESENT_SIZE>()
                        .unwrap()
                })
                .collect_array::<PRESENT_SIZE>()
                .unwrap();

            Present(present_shape)
        })
        .collect_array::<PRESENT_COUNT>()
        .unwrap();

    let regions = regions
        .iter()
        .map(|line| {
            let (region_size_str, present_counts_str) = line.split_once(':').unwrap();
            let (size_x, size_y) = region_size_str.split_once('x').unwrap();
            let region = Region::new(size_x.parse().unwrap(), size_y.parse().unwrap());
            let present_counts = present_counts_str
                .split_whitespace()
                .take(6)
                .map(|count| count.parse().unwrap())
                .collect_array::<PRESENT_COUNT>()
                .unwrap();
            (region, present_counts)
        })
        .collect();
    (presents, regions)
}

pub fn solve(
    region: &mut Region,
    presents: &[Present; PRESENT_COUNT],
    counts: &mut [u8; PRESENT_COUNT],
) -> bool {
    if counts.iter().all(|&c| c == 0) {
        return true;
    }

    for x in 0..=region.width() - PRESENT_SIZE {
        for y in 0..=region.height() - PRESENT_SIZE {
            for i in 0..PRESENT_COUNT {
                if counts[i] == 0 {
                    continue;
                }

                for orientation in Orientation::ALL {
                    let present = presents[i].rotate(orientation);

                    if region.place_present(x, y, present) {
                        counts[i] -= 1;

                        if solve(region, presents, counts) {
                            return true;
                        }

                        counts[i] += 1;
                        region.place_present_at(x, y, &present, false);
                    }
                }
            }
        }
    }

    false
}

pub fn part_one(input: &str) -> Option<u64> {
    // let (presents, regions) = parse_input(input);
    //
    // regions
    //     .into_iter()
    //     .try_fold(0, |mut acc, (region, present_counts)| {
    //         let mut queue: VecDeque<(Region, [u8; PRESENT_COUNT])> =
    //             VecDeque::from(vec![(region, present_counts)]);
    //
    //         let mut found_solution = false;
    //
    //         while let Some((region, counts)) = queue.pop_front() {
    //             if counts.iter().all(|&c| c == 0) {
    //                 found_solution = true;
    //                 break;
    //             }
    //             for x in 0..=region.width() - PRESENT_SIZE {
    //                 for y in 0..=region.height() - PRESENT_SIZE {
    //                     for i in 0..PRESENT_COUNT {
    //                         if counts[i] > 0 {
    //                             let present = presents[i];
    //                             for orientation in Orientation::ALL {
    //                                 let mut r = region;
    //                                 if r.place_present(x, y, present.rotate(orientation)) {
    //                                     let mut c = counts;
    //                                     c[i] -= 1;
    //                                     queue.push_back((r, c));
    //                                 }
    //                             }
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //
    //         println!("Finished region");
    //         if found_solution {
    //             acc += 1;
    //         }
    //
    //         Some(acc)
    //     })
    let (presents, regions) = parse_input(input);

    let mut acc = 0;

    for (mut region, mut counts) in regions {
        if solve(&mut region, &presents, &mut counts) {
            acc += 1;
        }
        println!("finished region");
    }

    Some(acc)
}

pub fn part_two(_input: &str) -> Option<u64> {
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
        let (presents, regions) =
            parse_input(&advent_of_code::template::read_file("examples", DAY));
        for present in presents {
            println!("{present:?}");
        }
        for (region, presents) in regions {
            println!("{region:?}");
            println!("{presents:?}");
        }
        // assert!(false);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
