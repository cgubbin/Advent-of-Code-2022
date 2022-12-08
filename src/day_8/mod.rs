use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::ops::Range;

#[derive(Debug, Hash, PartialEq, Eq)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Ord for Coordinate {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.x, self.y).cmp(&(other.x, other.y))
    }
}

impl PartialOrd for Coordinate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse(input: &str) -> BTreeMap<Coordinate, u8> {
    let mut items = BTreeMap::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, ch)| {
            items.insert(Coordinate { x, y }, ch as u8 - 48);
        })
    });

    items
}

fn visible_from_one_direction_x(
    range: Range<usize>,
    y: usize,
    height: u8,
    trees: &BTreeMap<Coordinate, u8>,
) -> bool {
    range
        .map(|x| Coordinate { x, y })
        .map(|c| trees[&c])
        .all(|other_height| other_height < height)
}

fn visible_from_one_direction_y(
    range: Range<usize>,
    x: usize,
    height: u8,
    trees: &BTreeMap<Coordinate, u8>,
) -> bool {
    range
        .map(|y| Coordinate { x, y })
        .map(|c| trees[&c])
        .all(|other_height| other_height < height)
}

fn is_on_edge(
    coordinate: &Coordinate,
    Coordinate {
        x: corner_x,
        y: corner_y,
    }: &Coordinate,
) -> bool {
    match coordinate {
        Coordinate { x: 0, .. } => true,
        Coordinate { y: 0, .. } => true,
        Coordinate { x, .. } if x == corner_x => true,
        Coordinate { y, .. } if y == corner_y => true,
        _ => false,
    }
}

fn visibility(trees: &BTreeMap<Coordinate, u8>) -> usize {
    let last_coordinate = trees.last_key_value().expect("Empty tree").0;
    trees.iter().fold(0, |sum, (coordinate, &height)| {
        let visible = if is_on_edge(coordinate, last_coordinate) {
            true
        } else {
            visible_from_one_direction_x(0..coordinate.x, coordinate.y, height, trees)
                || visible_from_one_direction_x(
                    (coordinate.x + 1)..(last_coordinate.x + 1),
                    coordinate.y,
                    height,
                    trees,
                )
                || visible_from_one_direction_y(0..coordinate.y, coordinate.x, height, trees)
                || visible_from_one_direction_y(
                    (coordinate.y + 1)..(last_coordinate.y + 1),
                    coordinate.x,
                    height,
                    trees,
                )
        };

        sum + visible as usize
    })
}

fn find_max<I>(iter: I) -> Option<I::Item>
where
    I: Iterator,
    I::Item: Ord,
{
    iter.reduce(|accum, item| if accum >= item { accum } else { item })
}

fn scenic_score_x(
    Range { start, end }: Range<usize>,
    y: usize,
    height: u8,
    trees: &BTreeMap<Coordinate, u8>,
) -> usize {
    let n = if start < end {
        end - start
    } else {
        start - end
    };
    let range = if start < end {
        Box::new(start..end) as Box<dyn Iterator<Item = _>>
    } else {
        Box::new((end..start).rev())
    };
    let mut last_height = 0;
    match range
        .map(|x| Coordinate { x, y })
        .map(|c| trees[&c])
        .take_while(|&other_height| {
            last_height = other_height;
            other_height < height
        })
        .count()
    {
        0 => 1,
        x if x == n => {
            if last_height == height {
                x + 1
            } else {
                x
            }
        }
        x => {
            if last_height == height {
                x + 1
            } else {
                x + 1
            }
        }
    }
}

fn scenic_score_y(
    Range { start, end }: Range<usize>,
    x: usize,
    height: u8,
    trees: &BTreeMap<Coordinate, u8>,
) -> usize {
    let n = if start < end {
        end - start
    } else {
        start - end
    };
    let range = if start < end {
        Box::new(start..end) as Box<dyn Iterator<Item = _>>
    } else {
        Box::new((end..start).rev())
    };
    let mut last_height = 0;
    match range
        .map(|y| Coordinate { x, y })
        .map(|c| trees[&c])
        .take_while(|&other_height| {
            last_height = other_height;
            other_height < height
        })
        .count()
    {
        0 => 1,
        x if x == n => {
            if last_height == height {
                x + 1
            } else {
                x
            }
        }
        x => {
            if last_height == height {
                x + 1
            } else {
                x + 1
            }
        }
    }
}

fn scenic_score(trees: &BTreeMap<Coordinate, u8>) -> usize {
    let last_coordinate = trees.last_key_value().expect("Empty tree").0;

    find_max(trees.iter().map(|(coordinate, &height)| {
        let score = if is_on_edge(coordinate, &last_coordinate) {
            0
        } else {
            scenic_score_x((coordinate.x)..0, coordinate.y, height, trees)
                * scenic_score_x(
                    (coordinate.x + 1)..(last_coordinate.x + 1),
                    coordinate.y,
                    height,
                    trees,
                )
                * scenic_score_y((coordinate.y)..0, coordinate.x, height, trees)
                * scenic_score_y(
                    (coordinate.y + 1)..(last_coordinate.y + 1),
                    coordinate.x,
                    height,
                    trees,
                )
        };
        score
    }))
    .expect("The scores were empty...")
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn day_8_test_1() {
        let input = include_str!("test").trim();
        let trees = parse(input);
        let score = visibility(&trees);
        assert_eq!(score, 21);
    }

    #[test]
    fn day_8_challenge_1() {
        let input = include_str!("input").trim();
        let trees = parse(input);
        let score = visibility(&trees);
        assert_eq!(score, 1840);
    }

    #[test]
    fn day_8_test_2() {
        let input = include_str!("test").trim();
        let trees = parse(input);
        let score = scenic_score(&trees);
        assert_eq!(score, 8);
    }

    #[test]
    fn day_8_challenge_2() {
        let input = include_str!("input").trim();
        let trees = parse(input);
        let score = scenic_score(&trees);
        assert_eq!(score, 405769);
    }
}
