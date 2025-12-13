use advent_of_code::dlx::Dlx;
use itertools::{Either, Itertools};
use rayon::prelude::*;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt::Debug;

advent_of_code::solution!(12);

pub const PRESENT_SIZE: usize = 3;
pub const PRESENT_COUNT: usize = 6;

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

#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct Present(pub [[bool; PRESENT_SIZE]; PRESENT_SIZE]);

impl Present {
    #[inline(always)]
    pub fn rotate(&self, orientation: Orientation) -> Present {
        let mut out = [[false; PRESENT_SIZE]; PRESENT_SIZE];

        for (i, row) in out.iter_mut().enumerate() {
            for (j, col) in row.iter_mut().enumerate() {
                *col = match orientation {
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

fn present_cells(p: Present) -> usize {
    p.0.iter().flatten().filter(|&&b| b).count()
}

pub const OFFSET_COUNT: usize = PRESENT_SIZE * PRESENT_SIZE;

pub type Offsets = [Option<(usize, usize)>; OFFSET_COUNT];

fn offsets(p: Present) -> [Option<(usize, usize)>; OFFSET_COUNT] {
    (0..PRESENT_SIZE)
        .flat_map(|y| {
            (0..PRESENT_SIZE).map(move |x| {
                if p.0[y][x] {
                    return Some((x, y));
                }
                None
            })
        })
        .sorted_unstable_by(|item, other| match (item, other) {
            (Some(x), Some(y)) => x.cmp(y),
            (Some(_), None) => Ordering::Less,
            (None, Some(_)) => Ordering::Greater,
            (None, None) => Ordering::Equal,
        })
        .collect_array()
        .unwrap()
}

pub const VARIATION_COUNT: usize = OFFSET_COUNT * Orientation::ALL.len();

pub type UniqueVariants = [Option<Offsets>; VARIATION_COUNT];

fn unique_variants(
    p: Present,
) -> UniqueVariants {
    let mut seen = HashSet::<[Option<(usize, usize)>; OFFSET_COUNT]>::new();
    let mut out = [None; VARIATION_COUNT];

    for (i, &o) in Orientation::ALL.iter().enumerate() {
        let off = offsets(p.rotate(o));
        if seen.insert(off) {
            out[i] = Some(off);
        }
    }
    out
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
pub struct Region(u8, u8);

impl Region {
    pub fn new(w: usize, h: usize) -> Self {
        Region(w as u8, h as u8)
    }

    #[inline(always)]
    pub fn width(&self) -> usize {
        self.0 as usize
    }

    #[inline(always)]
    pub fn height(&self) -> usize {
        self.1 as usize
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
    
    let mut instance_types = Vec::new();
    for (t, &count) in counts.iter().enumerate() {
        for _ in 0..count {
            instance_types.push(t);
        }
    }

    if instance_types.is_empty() {
        return true;
    }

    let num_instances = instance_types.len();
    let num_cells = w * h;

    let mut dlx = Dlx::new(num_instances, num_cells);

    let variants: [UniqueVariants; PRESENT_COUNT] = presents
        .iter()
        .copied()
        .map(unique_variants)
        .collect_array()
        .unwrap();

    let mut row_id = 0;
    let mut cols = Vec::<usize>::with_capacity(10);

    for (instance_id, &t) in instance_types.iter().enumerate() {
        for var in variants[t].iter().take_while(|v| v.is_some()).flatten() {
            for y0 in 0..=h - PRESENT_SIZE {
                for x0 in 0..=w - PRESENT_SIZE {
                    cols.clear();
                    cols.push(instance_id);

                    for &(dx, dy) in var.iter().take_while(|o| o.is_some()).flatten() {
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

pub fn part_one(input: &str) -> Option<u64> {
    let (presents, regions) = parse_input(input);

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
