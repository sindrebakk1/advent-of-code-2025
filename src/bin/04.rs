use itertools::{Itertools, multizip};

advent_of_code::solution!(4);

pub fn parse_input(input: &str) -> Vec<Vec<bool>> {
    let map: Vec<Vec<bool>> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().map(|c| c == '@').collect())
        .collect();

    let row_width = map[0].len();
    let map = std::iter::once(vec![false; row_width]).chain(map);
    let map = map.chain(std::iter::once(vec![false; row_width]));

    map.map(|row| {
        let row = std::iter::once(false).chain(row);
        row.chain(std::iter::once(false)).collect()
    })
    .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let map = parse_input(input);

    let res = map
        .iter()
        .tuple_windows()
        .fold(0, |mut accessible, (top, mid, bottom)| {
            for ((nw, n, ne), (w, c, e), (sw, s, se)) in multizip((
                top.iter().tuple_windows(),
                mid.iter().tuple_windows(),
                bottom.iter().tuple_windows(),
            )) {
                if *c {
                    let count = [*nw, *n, *ne, *w, *e, *sw, *s, *se].iter().fold(
                        0u8,
                        |mut adjacent, &curr| {
                            if curr {
                                adjacent += 1
                            };
                            adjacent
                        },
                    );
                    if count < 4 {
                        accessible += 1;
                    }
                }
            }

            accessible
        });

    Some(res)
}

pub fn remove_rolls(map: &mut [Vec<bool>]) -> Option<u64> {
    let mut score = 0;
    for x in 1..map.len() - 1 {
        for y in 1..map[x].len() - 1 {
            if map[x][y] {
                let count = [
                    map[x - 1][y - 1],
                    map[x][y - 1],
                    map[x + 1][y - 1],
                    map[x - 1][y],
                    map[x + 1][y],
                    map[x - 1][y + 1],
                    map[x][y + 1],
                    map[x + 1][y + 1],
                ]
                .iter()
                .fold(0u64, |mut adjacent, &curr| {
                    if curr {
                        adjacent += 1;
                    }
                    adjacent
                });
                if count < 4 {
                    score += 1;
                    map[x][y] = false;
                }
            }
        }
    }

    if score == 0 {
        return None;
    }
    Some(score)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut map = parse_input(input);
    let mut score = 0;
    while let Some(removed) = remove_rolls(&mut map) {
        score += removed;
    }
    Some(score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
