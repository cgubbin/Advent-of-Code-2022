#[cfg(test)]
mod tests {
    use std::{
        cmp::Ordering,
        collections::{BTreeMap, BTreeSet},
    };

    #[derive(Copy, Clone, Debug)]
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

    impl PartialEq for Coordinate {
        fn eq(&self, other: &Self) -> bool {
            (self.x == other.x) && (self.y == other.y)
        }
    }

    impl Eq for Coordinate {}

    impl Coordinate {
        fn neighbours(&self) -> impl Iterator<Item = Coordinate> {
            [
                (self.x as isize + 1, self.y as isize),
                (self.x as isize - 1, self.y as isize),
                (self.x as isize, self.y as isize + 1),
                (self.x as isize, self.y as isize - 1),
            ]
            .into_iter()
            .filter(|(x, y)| *x >= 0 && *y >= 0)
            .map(|(x, y)| Self {
                x: x as usize,
                y: y as usize,
            })
        }
    }

    #[derive(Debug)]
    struct Map {
        start: Coordinate,
        finish: Coordinate,
        heights: BTreeMap<Coordinate, u8>,
    }

    #[derive(Copy, Clone, Debug)]
    struct PathSegment(Coordinate, u16);

    impl PartialEq for PathSegment {
        fn eq(&self, other: &Self) -> bool {
            self.1 == other.1
        }
    }

    impl PartialOrd for PathSegment {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            use std::cmp::Reverse;

            Reverse(self.1).partial_cmp(&Reverse(other.1))
        }
    }
    impl Eq for PathSegment {}
    impl Ord for PathSegment {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            use std::cmp::Reverse;

            Reverse(self.1).cmp(&Reverse(other.1))
        }
    }

    fn parse(input: &str) -> Map {
        let mut start = None;
        let mut finish = None;
        let heights = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, ch)| match ch {
                        'S' => {
                            start = Some(Coordinate { x, y });
                            (Coordinate { x, y }, 1)
                        }
                        'E' => {
                            finish = Some(Coordinate { x, y });
                            (Coordinate { x, y }, 26)
                        }
                        ch => (Coordinate { x, y }, ch as u8 - 96),
                    })
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect();
        Map {
            start: start.unwrap(),
            finish: finish.unwrap(),
            heights,
        }
    }

    impl Map {
        fn shortest_path(&self, start: Coordinate) -> Option<u16> {
            use std::collections::BinaryHeap;
            let mut heap = BinaryHeap::from_iter([PathSegment(start, 0)].iter().copied());
            let mut used: BTreeSet<Coordinate> = BTreeSet::new();
            used.insert(start);

            while !heap.is_empty() {
                let PathSegment(coordinate, cost) = heap.pop().unwrap();

                if coordinate == self.finish {
                    return Some(cost);
                }

                let height = self
                    .heights
                    .get(&coordinate)
                    .expect("Coordinate not found in heights");

                for coordinate in coordinate.neighbours() {
                    if !used.contains(&coordinate) {
                        if let Some(&next_height) = self.heights.get(&coordinate) {
                            if height + 1 >= next_height {
                                used.insert(coordinate);
                                heap.push(PathSegment(coordinate, cost + 1));
                            }
                        }
                    }
                }
            }
            None
        }
    }

    #[test]
    fn day_12_test_1() {
        let input = include_str!("test").trim();

        let height_map = parse(input);

        let result = height_map.shortest_path(height_map.start).unwrap();

        assert_eq!(result, 31);
    }

    #[test]
    fn day_12_challenge_1() {
        let input = include_str!("input").trim();

        let height_map = parse(input);

        let result = height_map.shortest_path(height_map.start).unwrap();

        assert_eq!(result, 534);
    }

    #[test]
    fn day_12_test_2() {
        let input = include_str!("test").trim();
        let height_map = parse(input);

        let result = height_map
            .heights
            .iter()
            .filter(|(_, v)| **v == 1)
            .flat_map(|(k, _)| height_map.shortest_path(*k))
            .min()
            .unwrap();

        assert_eq!(result, 29);
    }

    #[test]
    fn day_12_challenge_2() {
        let input = include_str!("input").trim();
        let height_map = parse(input);

        let result = height_map
            .heights
            .iter()
            .filter(|(_, v)| **v == 1)
            .flat_map(|(k, _)| height_map.shortest_path(*k))
            .min()
            .unwrap();

        assert_eq!(result, 525);
    }
}
