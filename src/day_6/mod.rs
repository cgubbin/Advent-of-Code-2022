fn marker(signal: &[char], marker_length: usize) -> usize {
    match signal
        .windows(marker_length)
        .enumerate()
        .find(|(_, chars)| {
            let mut chars = chars.to_vec();
            chars.sort();
            chars.dedup();
            chars.into_iter().count() == marker_length
        }) {
        Some((index, _)) => index + marker_length,
        _ => unreachable!("Malformed Input"),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn day_6_challenge_1_test_1() {
        let input = include_str!("test1").trim().chars().collect::<Vec<char>>();

        let marker_position = marker(&input, 4);

        assert_eq!(marker_position, 7);
    }

    #[test]
    fn day_6_challenge_1_test_2() {
        let input = include_str!("test2").trim().chars().collect::<Vec<char>>();

        let marker_position = marker(&input, 4);

        assert_eq!(marker_position, 5);
    }

    #[test]
    fn day_6_challenge_1_test_3() {
        let input = include_str!("test3").trim().chars().collect::<Vec<char>>();

        let marker_position = marker(&input, 4);

        assert_eq!(marker_position, 6);
    }

    #[test]
    fn day_6_challenge_1_test_4() {
        let input = include_str!("test4").trim().chars().collect::<Vec<char>>();

        let marker_position = marker(&input, 4);

        assert_eq!(marker_position, 10);
    }

    #[test]
    fn day_6_challenge_1_test_5() {
        let input = include_str!("test5").trim().chars().collect::<Vec<char>>();

        let marker_position = marker(&input, 4);

        assert_eq!(marker_position, 11);
    }

    #[test]
    fn day_6_challenge_1() {
        let input = include_str!("input").trim().chars().collect::<Vec<char>>();

        let marker_position = marker(&input, 4);

        assert_eq!(marker_position, 1538);
    }

    #[test]
    fn day_6_challenge_2_test_1() {
        let input = include_str!("test1").trim().chars().collect::<Vec<char>>();

        let marker_position = marker(&input, 14);

        assert_eq!(marker_position, 19);
    }

    #[test]
    fn day_6_challenge_2_test_2() {
        let input = include_str!("test2").trim().chars().collect::<Vec<char>>();

        let marker_position = marker(&input, 14);

        assert_eq!(marker_position, 23);
    }

    #[test]
    fn day_6_challenge_2_test_3() {
        let input = include_str!("test3").trim().chars().collect::<Vec<char>>();

        let marker_position = marker(&input, 14);

        assert_eq!(marker_position, 23);
    }

    #[test]
    fn day_6_challenge_2_test_4() {
        let input = include_str!("test4").trim().chars().collect::<Vec<char>>();

        let marker_position = marker(&input, 14);

        assert_eq!(marker_position, 29);
    }

    #[test]
    fn day_6_challenge_2_test_5() {
        let input = include_str!("test5").trim().chars().collect::<Vec<char>>();

        let marker_position = marker(&input, 14);

        assert_eq!(marker_position, 26);
    }

    #[test]
    fn day_6_challenge_2() {
        let input = include_str!("input").trim().chars().collect::<Vec<char>>();

        let marker_position = marker(&input, 14);

        assert_eq!(marker_position, 2315);
    }
}
