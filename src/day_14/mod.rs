use itertools::{EitherOrBoth::*, Itertools};
use std::{cmp::Ordering, collections::BTreeSet, fmt::Display, str::FromStr};

#[derive(Copy, Clone, Debug)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Ord for Coordinate {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.y, self.x).cmp(&(other.y, other.x))
    }
}

impl PartialOrd for Coordinate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        (self.x == other.x) && (self.y == other.y)
    }
}

impl Eq for Coordinate {}

#[derive(Clone, Debug)]
struct LineSegment {
    start: Coordinate,
    finish: Coordinate,
}

impl LineSegment {
    fn from_coords(a: Coordinate, b: Coordinate) -> Self {
        match a < b {
            true => Self {
                start: a,
                finish: b,
            },
            false => Self {
                start: b,
                finish: a,
            },
        }
    }

    fn generate_coordinates(&self) -> Vec<Coordinate> {
        (self.start.x..(self.finish.x + 1))
            .zip_longest(self.start.y..(self.finish.y + 1))
            .map(|pair| match pair {
                Both(x, y) => Coordinate { x, y },
                Left(x) => Coordinate { x, y: self.start.y },
                Right(y) => Coordinate { x: self.start.x, y },
            })
            .collect()
    }
}

impl FromStr for Coordinate {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(',') {
            Some((x, y)) => Ok(Self {
                x: x.parse::<usize>().unwrap(),
                y: y.parse::<usize>().unwrap(),
            }),
            _ => Err("Malformed input".to_string()),
        }
    }
}

fn decode(input: &str) -> BTreeSet<Coordinate> {
    input
        .lines()
        .flat_map(|line| {
            let endpoints = line
                .split(" -> ")
                .map(|c| c.parse::<Coordinate>().unwrap())
                .collect::<Vec<_>>();
            endpoints
                .windows(2)
                .map(|endpoints| LineSegment::from_coords(endpoints[0], endpoints[1]))
                .flat_map(|line| line.generate_coordinates())
                .collect::<Vec<_>>()
        })
        .collect()
}

struct Cave {
    rocks: BTreeSet<Coordinate>,
    sum: BTreeSet<Coordinate>,
    sand: BTreeSet<Coordinate>,
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let min_x = self.sum.iter().min_by(|a, b| a.x.cmp(&b.x)).unwrap().x;
        let max_x = self.sum.iter().max_by(|a, b| a.x.cmp(&b.x)).unwrap().x;

        let string = (self.sum.first().unwrap().y..(self.sum.last().unwrap().y + 1))
            .map(|y| {
                let mut figure = (min_x..(max_x + 1))
                    .map(|x| {
                        if let Some(_) = self.rocks.get(&Coordinate { x, y }) {
                            '\u{2593}'
                        } else if let Some(_) = self.sand.get(&Coordinate { x, y }) {
                            '*'
                        } else {
                            '\u{2591}'
                        }
                    })
                    .collect::<String>();
                figure.push_str(format!("  {y}").as_str());
                figure
            })
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{}", string)
    }
}

impl Cave {
    fn is_outside(&self, position: Coordinate) -> bool {
        let min_x = self.rocks.iter().min_by(|a, b| a.x.cmp(&b.x)).unwrap().x;
        let max_x = self.rocks.iter().max_by(|a, b| a.x.cmp(&b.x)).unwrap().x;
        let min_y = self.rocks.last().unwrap().y;

        if (position.x < min_x) || (position.x > max_x) || (position.y > min_y) {
            true
        } else {
            false
        }
    }
    fn fall(&mut self, sand_position: Coordinate) -> bool {
        let mut sand_position = Some(sand_position);
        while let Some(position) = sand_position {
            sand_position = self.fall_once(position);
            if self.is_outside(position) {
                return false;
            }
        }
        true
    }

    fn fall_once(&mut self, sand_position: Coordinate) -> Option<Coordinate> {
        if let Some(_) = self.sum.get(&Coordinate {
            x: sand_position.x,
            y: sand_position.y + 1,
        }) {
            if let Some(_) = self.sum.get(&Coordinate {
                x: sand_position.x - 1,
                y: sand_position.y + 1,
            }) {
                if let Some(_) = self.sum.get(&Coordinate {
                    x: sand_position.x + 1,
                    y: sand_position.y + 1,
                }) {
                    self.sand.insert(sand_position);
                    self.sum.insert(sand_position);
                    return None;
                } else {
                    return Some(Coordinate {
                        x: sand_position.x + 1,
                        y: sand_position.y + 1,
                    });
                }
            } else {
                return Some(Coordinate {
                    x: sand_position.x - 1,
                    y: sand_position.y + 1,
                });
            }
        } else {
            return Some(Coordinate {
                x: sand_position.x,
                y: sand_position.y + 1,
            });
        }
    }

    fn fall_to_floor(&mut self, sand_position: Coordinate) -> bool {
        let floor_row = self.rocks.last().unwrap().y + 2;
        let mut sand_position = Some(sand_position);
        let mut num_moves = 0;
        while let Some(position) = sand_position {
            sand_position = self.fall_once(position);
            num_moves += 1;
            if position.y == floor_row - 1 {
                self.sand.insert(position);
                self.sum.insert(position);
                return true;
            }
        }
        if num_moves == 1 {
            false
        } else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn part_1(rocks: BTreeSet<Coordinate>) -> usize {
        let mut cave = Cave {
            rocks: rocks.clone(),
            sum: rocks,
            sand: BTreeSet::new(),
        };
        let fall_point = Coordinate { x: 500, y: 0 };

        let mut n = 0;
        while cave.fall(fall_point) {
            n += 1;
        }
        // println!("{cave}");
        n
    }

    fn part_2(rocks: BTreeSet<Coordinate>) -> usize {
        let mut cave = Cave {
            rocks: rocks.clone(),
            sum: rocks,
            sand: BTreeSet::new(),
        };
        let fall_point = Coordinate { x: 500, y: 0 };

        let mut n = 1;
        while cave.fall_to_floor(fall_point) {
            n += 1;
        }
        // println!("{cave}");
        n
    }

    #[test]
    fn day_14_test_1() {
        let input = include_str!("test").trim();

        let rocks = decode(input);
        let result = part_1(rocks);

        assert_eq!(result, 24);
    }

    #[test]
    fn day_14_challenge_1() {
        let input = include_str!("input").trim();

        let rocks = decode(input);
        let result = part_1(rocks);

        assert_eq!(result, 832);
    }

    #[test]
    fn day_14_test_2() {
        let input = include_str!("test").trim();

        let rocks = decode(input);
        let result = part_2(rocks);

        assert_eq!(result, 93);
    }

    #[test]
    fn day_14_challenge_2() {
        let input = include_str!("input").trim();

        let rocks = decode(input);
        let result = part_2(rocks);

        assert_eq!(result, 27601);
    }
}
