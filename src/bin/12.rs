use advent_of_code::dlx::Dlx;
use itertools::{Either, Itertools};
use rayon::prelude::*;
use regex::Regex;
use std::collections::HashSet;
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
pub struct Region([[bool; 50]; 50], (u8, u8));

impl Region {
    pub fn new(w: usize, h: usize) -> Self {
        Region([[false; 50]; 50], (w as u8, h as u8))
    }

    pub fn place_present(&mut self, x: usize, y: usize, present: Present) -> bool {
        for i in 0..PRESENT_SIZE {
            for j in 0..PRESENT_SIZE {
                if !present.0[i][j] {
                    continue;
                }
                if self.0[i + y][j + x] {
                    return false;
                }
            }
        }

        for i in 0..PRESENT_SIZE {
            for j in 0..PRESENT_SIZE {
                if present.0[i][j] {
                    self.0[i + y][j + x] = true;
                }
            }
        }
        true
    }

    #[inline(always)]
    pub fn width(&self) -> usize {
        self.1.0 as usize
    }

    #[inline(always)]
    pub fn height(&self) -> usize {
        self.1.1 as usize
    }
}

impl Debug for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height() {
            for x in 0..self.width() {
                match self.0[y][x] {
                    true => f.write_str("# "),
                    false => f.write_str(". "),
                }?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

fn present_cells(p: Present) -> usize {
    p.0.iter().flatten().filter(|&&b| b).count()
}

fn offsets(p: Present) -> Vec<(usize, usize)> {
    let mut v = Vec::new();
    for y in 0..PRESENT_SIZE {
        for x in 0..PRESENT_SIZE {
            if p.0[y][x] {
                v.push((x, y));
            }
        }
    }
    v
}

fn unique_variants(p: Present) -> Vec<Vec<(usize, usize)>> {
    let mut seen = HashSet::<Vec<(usize, usize)>>::new();
    let mut out = Vec::new();

    for o in Orientation::ALL {
        let mut off = offsets(p.rotate(o));
        off.sort_unstable();
        if seen.insert(off.clone()) {
            out.push(off);
        }
    }
    out
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
    region: Region,
    presents: [Present; PRESENT_COUNT],
    counts: [u8; PRESENT_COUNT],
) -> bool {
    let w = region.width();
    let h = region.height();

    // hard prune: area
    let mut required = 0usize;
    for i in 0..PRESENT_COUNT {
        required += present_cells(presents[i]) * counts[i] as usize;
    }
    if required > w * h {
        return false;
    }

    // expand counts → instance list
    let mut instance_types = Vec::new();
    for t in 0..PRESENT_COUNT {
        for _ in 0..counts[t] {
            instance_types.push(t);
        }
    }

    if instance_types.is_empty() {
        return true;
    }

    let num_instances = instance_types.len();
    let num_cells = w * h;

    // DLX: primary = instances, secondary = cells
    let mut dlx = Dlx::new(num_instances, num_cells);

    let variants: Vec<Vec<Vec<(usize, usize)>>> =
        presents.iter().copied().map(unique_variants).collect();

    let mut row_id = 0usize;
    let mut cols = Vec::<usize>::with_capacity(10);

    for (instance_id, &t) in instance_types.iter().enumerate() {
        for var in &variants[t] {
            for y0 in 0..=h - PRESENT_SIZE {
                for x0 in 0..=w - PRESENT_SIZE {
                    cols.clear();
                    cols.push(instance_id); // primary column

                    for &(dx, dy) in var {
                        let x = x0 + dx;
                        let y = y0 + dy;
                        cols.push(num_instances + y * w + x);
                    }

                    dlx.add_row(row_id, &cols);
                    row_id += 1;
                }
            }
        }
    }

    dlx.solve_one().is_some()
}
// pub fn solve(
//     region: Region,
//     presents: [Present; PRESENT_COUNT],
//     counts: [u8; PRESENT_COUNT],
// ) -> bool {
//     if counts.iter().all(|&c| c == 0) {
//         return true;
//     }
//
//     for x in 0..=region.width() - PRESENT_SIZE {
//         for y in 0..=region.height() - PRESENT_SIZE {
//             if region.0[y][x] {
//                 continue;
//             }
//             for i in 0..PRESENT_COUNT {
//                 if counts[i] == 0 {
//                     continue;
//                 }
//
//                 let present = presents[i];
//                 for orientation in Orientation::ALL {
//                     let mut r = region;
//                     if r.place_present(x, y, present.rotate(orientation)) {
//                         println!("{r:?}");
//                         let mut c = counts;
//                         c[i] -= 1;
//                         if solve(r, presents, c) {
//                             return true;
//                         }
//                     }
//                 }
//             }
//         }
//     }
//
//     false
// }

// pub fn solve(
//     region: Region,
//     presents: [Present; PRESENT_COUNT],
//     counts: [u8; PRESENT_COUNT],
// ) -> bool {
//     // success: all presents placed
//     if counts.iter().all(|&c| c == 0) {
//         return true;
//     }
//
//     // choose the next present to place (first with count > 0)
//     let i = match (0..PRESENT_COUNT).find(|&i| counts[i] > 0) {
//         Some(i) => i,
//         None => return true,
//     };
//
//     let present = presents[i];
//
//     for orientation in Orientation::ALL {
//         let rotated = present.rotate(orientation);
//
//         for y in 0..=region.height() - PRESENT_SIZE {
//             for x in 0..=region.width() - PRESENT_SIZE {
//                 let mut r = region;
//                 if r.place_present(x, y, rotated) {
//                     let mut c = counts;
//                     c[i] -= 1;
//
//                     if solve(r, presents, c) {
//                         return true;
//                     }
//                 }
//             }
//         }
//     }
//
//     false
// }
//
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

    // let mut acc = 0;
    //
    // for (region, counts) in regions {
    //     if solve(region, presents, counts) {
    //         acc += 1;
    //     }
    //     println!("finished region");
    // }

    let acc: u64 = regions
        .par_iter()
        .map(|(region, counts)| solve(*region, presents, *counts) as u64)
        .sum();

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
