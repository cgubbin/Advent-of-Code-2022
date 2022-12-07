use std::collections::{HashMap, HashSet};

fn score(ch: char) -> u32 {
    if ch.is_lowercase() {
        ch as u32 - 96
    } else if ch.is_uppercase() {
        ch as u32 - 38
    } else {
        unreachable!("All items are represented by uppercase or lowercase letters")
    }
}

#[derive(Debug)]
struct Compartment(HashSet<char>);

impl Compartment {
    fn from_contents(contents: &str) -> Self {
        let mut set = HashSet::new();
        for ch in contents.chars() {
            set.insert(ch);
        }
        Self(set)
    }

    fn score_conflict(&self, other: &Self) -> Option<u32> {
        self.find_conflict(other).into_iter().last().map(score)
    }

    fn find_conflict(&self, other: &Self) -> Vec<char> {
        other
            .0
            .iter()
            .filter(|ch| self.0.contains(ch))
            .copied()
            .collect::<Vec<_>>()
    }

    fn find_badge(&self, others: &[Self]) -> Option<char> {
        let conflicts = others
            .iter()
            .flat_map(|other| self.find_conflict(other))
            .collect::<Vec<_>>();

        let mut occurrences = HashMap::new();

        for ch in conflicts {
            *occurrences.entry(ch).or_insert(1) += 1;
        }

        if occurrences.is_empty() {
            return None;
        }

        Some(
            occurrences
                .into_iter()
                .max_by_key(|&(_, count)| count)
                .map(|(val, _)| val)
                .expect("Cannot compute the mode of zero numbers"),
        )
    }
}

#[cfg(test)]
mod tests {

    use super::{score, Compartment};

    #[test]
    fn day_3_challenge_1() {
        let input = include_str!("input").trim();
        let result = input
            .split('\n')
            .map(|rucksack| {
                let number_of_items = rucksack.len() / 2;
                let contents = [&rucksack[..number_of_items], &rucksack[number_of_items..]];
                let compartment_1 = Compartment::from_contents(contents[0]);
                let compartment_2 = Compartment::from_contents(contents[1]);
                let conflict_score = compartment_1.score_conflict(&compartment_2);
                assert!(conflict_score.is_some());
                conflict_score.unwrap()
            })
            .sum::<u32>();
        assert_eq!(result, 7848);
    }

    #[test]
    fn day_3_challenge_2() {
        let input = include_str!("input").trim();
        let result = input
            .split('\n')
            .array_chunks()
            .map(|[rucksack_1, rucksack_2, rucksack_3]| {
                let rucksacks = [rucksack_1, rucksack_2, rucksack_3];
                let compartments = rucksacks
                    .iter()
                    .map(|contents| Compartment::from_contents(contents))
                    .collect::<Vec<_>>();
                let score = compartments[0].find_badge(&compartments[1..]).map(score);
                assert!(score.is_some());
                score.unwrap()
            })
            .sum::<u32>();
        assert_eq!(result, 2616);
    }
}
