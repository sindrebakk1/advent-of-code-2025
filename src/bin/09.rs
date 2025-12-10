use glam::IVec2;
use itertools::Itertools;
use std::cmp::{max, min};

advent_of_code::solution!(9);

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Orientation {
    Vertical,
    Horizontal,
}

#[derive(Copy, Clone, Debug)]
pub struct Edge {
    pub start: IVec2,
    pub end: IVec2,
    pub orientation: Orientation,
}

impl Edge {
    pub fn new(start: IVec2, end: IVec2) -> Edge {
        let orientation = match (start.x == end.x, start.y == end.y) {
            (true, false) => Orientation::Vertical,
            (false, true) => Orientation::Horizontal,
            (_, _) => panic!("invalid orientation, must be horizontal or vertical"),
        };
        Edge {
            start,
            end,
            orientation,
        }
    }

    pub fn intersects(&self, other: &Edge) -> bool {
        match (self.orientation, other.orientation) {
            (Orientation::Horizontal, Orientation::Vertical) => {
                let y_plane = self.start.y;
                let x_plane = other.start.x;
                ((self.start.x > x_plane && self.end.x < x_plane)
                    || (self.start.x < x_plane && self.end.x > x_plane))
                    && ((other.start.y > y_plane && other.end.y < y_plane)
                        || (other.start.y < y_plane && other.end.y > y_plane))
            }
            (Orientation::Vertical, Orientation::Horizontal) => {
                let y_plane = other.start.y;
                let x_plane = self.start.x;
                ((self.start.y > y_plane && self.end.y < y_plane)
                    || (self.start.y < y_plane && self.end.y > y_plane))
                    && ((other.start.x > x_plane && other.end.x < x_plane)
                        || (other.start.x < x_plane && other.end.x > x_plane))
            }
            (_, _) => false,
        }
    }
}

pub struct Polygon(pub Vec<Edge>);

pub fn parse_input(input: &str) -> Vec<IVec2> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split_once(',')
                .map(|(x, y)| IVec2::new(x.parse().unwrap(), y.parse().unwrap()))
                .unwrap()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    parse_input(input.trim())
        .into_iter()
        .tuple_combinations()
        .try_fold(0, |mut acc, (a, b)| {
            let area =
                ((b.x - a.x).unsigned_abs() as u64 + 1) * ((b.y - a.y).unsigned_abs() as u64 + 1);
            acc = max(acc, area);
            Some(acc)
        })
}

pub fn collect_edges(points: &[IVec2]) -> Vec<Edge> {
    let mut edges: Vec<Edge> = points
        .iter()
        .tuple_windows()
        .map(|(&a, &b)| Edge::new(a, b))
        .collect();
    edges.push(Edge::new(points[0], points[points.len() - 1]));
    edges
}

pub fn part_two(_input: &str) -> Option<u64> {
    let points: Vec<IVec2> = parse_input(_input);
    let edges = collect_edges(&points);

    let horizontal_edges: Vec<Edge> = edges.iter().copied().filter(|e| e.orientation == Orientation::Horizontal).collect();
    let vertical_edges: Vec<Edge> = edges.iter().copied().filter(|e| e.orientation == Orientation::Vertical).collect();

    points
        .into_iter()
        .tuple_combinations()
        .try_fold(0, |mut acc, (a, b)| {
            let (start_x, end_x) = (min(a.x, b.x), max(a.x, b.x));
            let (start_y, end_y) = (min(a.y, b.y), max(a.y, b.y));
            if start_x == end_x || start_y == end_y {
                return Some(acc);
            }
            for x in start_x..end_x {
                let ray = Edge::new(IVec2::new(x, start_y), IVec2::new(x, end_y));
                if horizontal_edges.iter().any(|e| ray.intersects(e)) {
                    return Some(acc);
                }
            }
            for y in start_y..end_y {
                let ray = Edge::new(IVec2::new(start_x, y), IVec2::new(end_x, y));
                if vertical_edges.iter().any(|e| ray.intersects(e)) {
                    return Some(acc);
                }
            }
            let area =
                ((b.x - a.x).unsigned_abs() as u64 + 1) * ((b.y - a.y).unsigned_abs() as u64 + 1);
            acc = max(acc, area);
            Some(acc)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edge_construction() {
        let _ = [
            Edge::new(IVec2::new(7,1), IVec2::new(11,1)),
            Edge::new(IVec2::new(11,1), IVec2::new(11,7)),
            Edge::new(IVec2::new(11,7), IVec2::new(9,7)),
            Edge::new(IVec2::new(9,7), IVec2::new(9,5)),
            Edge::new(IVec2::new(9,5), IVec2::new(2,5)),
            Edge::new(IVec2::new(2,5), IVec2::new(2,3)),
            Edge::new(IVec2::new(2,3), IVec2::new(7,3)),
            Edge::new(IVec2::new(7,3), IVec2::new(7,1)),
        ];
    }

    #[test]
    fn test_intersection() {
        let edge_a = Edge::new(IVec2::new(1, 6), IVec2::new(6, 6));
        let edge_b = Edge::new(IVec2::new(2, 8), IVec2::new(2, 4));
        let edge_c = Edge::new(IVec2::new(4, 6), IVec2::new(4, 10));

        // A | B
        assert!(edge_a.intersects(&edge_b));
        assert!(edge_b.intersects(&edge_a));

        // A | C
        assert!(!edge_a.intersects(&edge_c));
        assert!(!edge_c.intersects(&edge_a));

        // B | C
        assert!(!edge_b.intersects(&edge_c));
        assert!(!edge_c.intersects(&edge_b));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
