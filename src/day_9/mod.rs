#[cfg(test)]
mod tests {

    use std::{cmp::Ordering, collections::BTreeSet, ops::Sub};

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct Coordinate {
        x: i16,
        y: i16,
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

    impl Sub for Coordinate {
        type Output = Self;

        fn sub(self, other: Self) -> Self::Output {
            Self {
                x: self.x - other.x,
                y: self.y - other.y,
            }
        }
    }

    #[derive(Clone, Debug)]
    enum Direction {
        Left,
        Right,
        Up,
        Down,
    }

    impl Coordinate {
        fn move_in(&mut self, direction: &Direction) {
            match direction {
                Direction::Left => self.x -= 1,
                Direction::Right => self.x += 1,
                Direction::Up => self.y += 1,
                Direction::Down => self.y -= 1,
            }
        }

        fn follow(&mut self, last: &Self) {
            if (last.x - self.x).abs() >= 2 || (last.y - self.y).abs() >= 2 {
                self.x += (last.x - self.x).signum();
                self.y += (last.y - self.y).signum();
            }
        }
    }

    fn parse(input: &str) -> Vec<Direction> {
        input
            .lines()
            .map(|line| match line.split_once(" ") {
                Some((direction, repeats)) => {
                    let repeats = repeats.parse::<usize>().expect("Repeats can't be parsed");
                    vec![
                        match direction.to_uppercase().as_str() {
                            "L" => Direction::Left,
                            "R" => Direction::Right,
                            "U" => Direction::Up,
                            "D" => Direction::Down,
                            _ => unreachable!("Malformed Input"),
                        };
                        repeats
                    ]
                }
                _ => unreachable!("Malformed input"),
            })
            .flatten()
            .collect()
    }

    fn process(actions: &[Direction], number_of_knots: usize) -> usize {
        let mut positions = vec![Coordinate { x: 0, y: 0 }; number_of_knots];
        let mut visited = BTreeSet::new();

        visited.insert(positions.last().unwrap().clone());

        actions.iter().for_each(|direction| {
            // Head
            positions[0].move_in(direction);

            for knot in 1..number_of_knots {
                let last = positions[knot - 1].clone();
                positions[knot].follow(&last);
            }

            visited.insert(positions.last().unwrap().clone());
        });

        visited.len()
    }

    #[test]
    fn day_9_test_1() {
        let input = include_str!("test").trim();
        let actions = parse(input);
        let result = process(&actions, 2);
        assert_eq!(result, 13);
    }

    #[test]
    fn day_9_challenge_1() {
        let input = include_str!("input").trim();
        let actions = parse(input);
        let result = process(&actions, 2);
        assert_eq!(result, 6503);
    }

    #[test]
    fn day_9_test_2() {
        let input = include_str!("test").trim();
        let actions = parse(input);
        let result = process(&actions, 10);
        assert_eq!(result, 1);
    }

    #[test]
    fn day_9_test_3() {
        let input = include_str!("test2").trim();
        let actions = parse(input);
        let result = process(&actions, 10);
        assert_eq!(result, 36);
    }

    #[test]
    fn day_9_challenge_2() {
        let input = include_str!("input").trim();
        let actions = parse(input);
        let result = process(&actions, 10);
        assert_eq!(result, 2724);
    }
}
