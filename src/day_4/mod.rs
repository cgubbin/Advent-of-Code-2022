#[cfg(test)]
mod tests {

    use std::cmp::{max, min};
    use std::ops::Range;

    fn convert_to_range(range: &str) -> Range<u16> {
        match range.split_once('-') {
            Some((start, end)) => Range {
                start: start.parse().expect("Failed to parse to u16"),
                end: end.parse().expect("Failed to parse to u16"),
            },
            _ => unreachable!("Malformed input, should be a `-` delimited tuple"),
        }
    }

    fn intersection(range_1: &Range<u16>, range_2: &Range<u16>) -> Option<Range<u16>> {
        match (range_2.start > range_1.end) | (range_1.start > range_2.end) {
            true => None,
            false => Some(max(range_1.start, range_2.start)..min(range_1.end, range_2.end)),
        }
    }

    #[test]
    fn day_4_challenge_1() {
        let input = include_str!("input").trim();

        let result = input
            .split('\n')
            .filter_map(|row| {
                match row.split_once(',') {
                    Some((range_1, range_2)) => {
                        let range_1 = convert_to_range(range_1);
                        let range_2 = convert_to_range(range_2);
                        intersection(&range_1, &range_2)
                            .map(|int| (int == range_1) | (int == range_2))
                    }
                    _ => unreachable!("Malformed input, should be a `,` delimited tuple"),
                }
                .map(|result| result as u16)
            })
            .sum::<u16>();

        assert_eq!(result, 605)
    }

    #[test]
    fn day_4_challenge_2() {
        let input = include_str!("input").trim();

        let result = input
            .split('\n')
            .filter_map(|row| {
                match row.split_once(',') {
                    Some((range_1, range_2)) => {
                        let range_1 = convert_to_range(range_1);
                        let range_2 = convert_to_range(range_2);
                        intersection(&range_1, &range_2)
                    }
                    _ => unreachable!("Malformed input, should be a `,` delimited tuple"),
                }
                .map(|_| 1u16)
            })
            .sum::<u16>();

        assert_eq!(result, 914)
    }
}
