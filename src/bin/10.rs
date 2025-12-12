#![feature(portable_simd)]

use arrayvec::ArrayVec;
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashSet};
use std::fmt::{Debug, Write};
use std::simd::cmp::SimdPartialOrd;
use std::simd::u8x16;

advent_of_code::solution!(10);
const MAX_BUTTON_COUNT: usize = 16;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Button(u16);

impl Button {
    pub fn from_indices<T>(indices: T) -> Self
    where
        T: IntoIterator<Item = u8>,
    {
        let mut bitmask: u16 = 0;
        for index in indices {
            bitmask |= 1 << index;
        }
        Self(bitmask)
    }
}

impl Debug for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('(')?;

        let mut first = true;
        for i in 0..16 {
            if (self.0 & (1 << i)) != 0 {
                if !first {
                    f.write_str(", ")?;
                }
                first = false;

                write!(f, "{}", i)?;
            }
        }
        f.write_char(')')
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct LightBank(u16);

impl LightBank {
    pub const OFF: LightBank = LightBank(0);
    pub const ON: LightBank = LightBank(1);

    pub fn from_indices<T>(indices: T) -> Self
    where
        T: IntoIterator<Item = bool>,
    {
        let mut bitmask: u16 = 0;
        for (i, on) in indices.into_iter().enumerate() {
            if on {
                bitmask |= 1 << i;
            }
        }

        LightBank(bitmask)
    }

    pub fn apply_button(&mut self, button: Button) {
        self.0 ^= button.0;
    }

    pub fn with_applied_button(&self, button: &Button) -> Self {
        Self(self.0 ^ button.0)
    }
}

impl From<Vec<bool>> for LightBank {
    fn from(value: Vec<bool>) -> Self {
        Self::from_indices(value)
    }
}

impl Debug for LightBank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('[')?;
        for i in 0..16 {
            let c = if (self.0 >> i) & 1 == 1 { '#' } else { '.' };
            f.write_char(c)?;
        }
        f.write_char(']')
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct VoltageLevels(u128);

impl VoltageLevels {
    pub const ZERO: VoltageLevels = VoltageLevels(0);

    pub fn from_values<T>(values: T) -> Self
    where
        T: IntoIterator<Item = u8>,
    {
        let mut packed: u128 = 0;
        for (i, value) in values.into_iter().enumerate() {
            assert!(i < 16, "Maximum 16 values allowed");
            packed |= (value as u128) << (i * 8);
        }
        VoltageLevels(packed)
    }

    pub fn set(&mut self, index: u8, value: u8) {
        assert!(index < 16, "Index must be 0..15");

        let shift = index * 8;
        let mask = 0xFFu128 << shift;

        self.0 = (self.0 & !mask) | ((value as u128) << shift);
    }

    pub fn get(&self, index: u8) -> u8 {
        assert!(index < 16, "Index must be 0..15");
        ((self.0 >> (index * 8)) & 0xFF) as u8
    }
}

pub struct Machine {
    pub desired_state: LightBank,
    pub buttons: ArrayVec<Button, MAX_BUTTON_COUNT>,
}

impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let chars: Vec<char> = value.chars().collect();
        let (mut i, mut j) = (1, 2);
        while chars[j] != ']' {
            j += 1;
        }
        let desired_state_iter = value[i..j].chars().map(|c| match c {
            '.' => false,
            '#' => true,
            _ => panic!("Unexpected char {}", c),
        });
        let desired_state = LightBank::from_indices(desired_state_iter);

        i = j + 2;
        j = i + 1;
        while chars[j] != '{' {
            j += 1;
        }
        let buttons: ArrayVec<Button, MAX_BUTTON_COUNT> = value[i..j - 1]
            .split(' ')
            .map(|button_str| {
                let iter = button_str
                    .trim_matches(|c| c == '(' || c == ')')
                    .split(',')
                    .map(|idx_str| idx_str.parse::<u8>().unwrap());
                Button::from_indices(iter)
            })
            .collect();

        Machine {
            desired_state,
            buttons,
        }
    }
}

impl Debug for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fmt_bank =
            |f: &mut std::fmt::Formatter<'_>, lb: &LightBank, color: &str| -> std::fmt::Result {
                f.write_str(color)?;

                f.write_char('[')?;
                for i in 0..16 {
                    if (lb.0 & (1 << i)) != 0 {
                        f.write_char('#')?;
                    } else {
                        f.write_char('.')?;
                    }
                }
                f.write_char(']')?;

                f.write_str("\x1b[0m")
            };

        let fmt_button = |f: &mut std::fmt::Formatter<'_>, b: &Button| -> std::fmt::Result {
            f.write_char('(')?;
            let mut first = true;
            for i in 0..16 {
                if (b.0 & (1 << i)) != 0 {
                    if !first {
                        f.write_str(", ")?;
                    }
                    write!(f, "{}", i)?;
                    first = false;
                }
            }
            f.write_char(')')
        };

        writeln!(f, "+--------------------+")?;
        writeln!(f, "|       ElfCorp      |")?;
        write!(f, "| ")?;
        fmt_bank(f, &self.desired_state, "\x1b[90m")?;
        writeln!(f, " |")?;
        writeln!(f, "+--------------------+")?;
        writeln!(f, "|    ▼  BUTTONS  ▼   |")?;

        for (i, btn) in self.buttons.iter().enumerate() {
            write!(f, "| {}: ", i)?;
            fmt_button(f, btn)?;
            writeln!(f)?;
        }

        Ok(())
    }
}

pub fn parse_input_p1(input: &str) -> Vec<Machine> {
    input.trim().lines().map(Machine::from).collect()
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PossibleSolutionP1(pub u64, pub LightBank);

impl Ord for PossibleSolutionP1 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for PossibleSolutionP1 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    parse_input_p1(input)
        .iter()
        .try_fold(0, |mut acc, machine| {
            let mut heap = BinaryHeap::new();
            let mut seen = HashSet::new();
            for button in &machine.buttons {
                heap.push(Reverse(PossibleSolutionP1(
                    1,
                    LightBank::OFF.with_applied_button(button),
                )))
            }
            while let Some(Reverse(possible_solution)) = heap.pop() {
                if possible_solution.1 == machine.desired_state {
                    acc += possible_solution.0;
                    break;
                }
                for button in &machine.buttons {
                    let next_state = possible_solution.1.with_applied_button(button);
                    if seen.insert(next_state) {
                        heap.push(Reverse(PossibleSolutionP1(
                            possible_solution.0 + 1,
                            next_state,
                        )))
                    }
                }
            }
            Some(acc)
        })
}

pub fn parse_input_p2(input: &str) -> Vec<(u8x16, ArrayVec<u8x16, MAX_BUTTON_COUNT>)> {
    input
        .trim()
        .lines()
        .map(|line| {
            let chars: Vec<char> = line.chars().collect();
            let start = line.find(']').unwrap() + 2;
            let mut i = start + 1;

            while chars[i] != '{' {
                i += 1;
            }
            let buttons: ArrayVec<u8x16, MAX_BUTTON_COUNT> = line[start..i - 1]
                .split(' ')
                .map(|button_str| {
                    let mut mask: [u8; 16] = [0; 16];
                    button_str
                        .trim_matches(|c| c == '(' || c == ')')
                        .split(',')
                        .for_each(|idx_str| {
                            let idx = idx_str.parse::<usize>().unwrap();
                            mask[idx] = 1;
                        });
                    u8x16::from_array(mask)
                })
                .collect();

            let mut voltage_values: Vec<u8> = line[i + 1..]
                .trim_matches(|c| c == '{' || c == '}')
                .split(',')
                .map(|val_str| val_str.parse::<u8>().unwrap())
                .collect();
            voltage_values.resize(16, 0);
            (u8x16::from_slice(&voltage_values), buttons)
        })
        .collect()
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PossibleSolutionP2(u64, u8x16);

impl Ord for PossibleSolutionP2 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for PossibleSolutionP2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    parse_input_p2(input)
        .into_iter()
        .try_fold(0, |mut acc, (desired_voltages, buttons)| {
            println!("{:?} | {:?}", desired_voltages, buttons);
            let mut heap = BinaryHeap::new();
            let mut seen = HashSet::new();
            for button in &buttons {
                heap.push(Reverse(PossibleSolutionP2(
                    1,
                    u8x16::from_array([0; 16]) + button,
                )))
            }
            while let Some(Reverse(possible_solution)) = heap.pop() {
                if possible_solution.1 == desired_voltages {
                    acc += possible_solution.0;
                    break;
                }
                for button in &buttons {
                    let next_state = possible_solution.1 + button;

                    if seen.insert(next_state) && !next_state.simd_gt(desired_voltages).any() {
                        heap.push(Reverse(PossibleSolutionP2(
                            possible_solution.0 + 1,
                            next_state,
                        )))
                    }
                }
            }
            println!("finished machine");
            Some(acc)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input_p1() {
        let _ = parse_input_p1(&advent_of_code::template::read_file("examples", DAY));
    }

    #[test]
    fn test_parse_input_p2() {
        let _ = parse_input_p2(&advent_of_code::template::read_file("examples", DAY));
    }

    #[test]
    fn test_light_bank() {
        let light_bank = LightBank::OFF;
        let desired_state = LightBank::from_indices(vec![true, false, true, false]);
        let button = Button::from_indices(vec![0, 2]);
        assert_eq!(light_bank.with_applied_button(&button), desired_state);

        let light_bank = LightBank::from_indices(vec![true, false, true, false]);
        let desired_state = LightBank::OFF;
        let button = Button::from_indices(vec![0, 2]);
        assert_eq!(light_bank.with_applied_button(&button), desired_state);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}
