use std::cmp::Ordering;
use std::iter::Sum;

/// Wrapper representing an Elf, carrying `u64` calories.
#[derive(Debug, Eq)]
struct Elf(u64);

impl Ord for Elf {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for Elf {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Elf {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Sum for Elf {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        Self(iter.fold(0u64, |a, b| a + b.0))
    }
}

impl Elf {
    /// Collects input from file, creating a Vec of Elf
    ///
    /// The file contains a single column of integers.
    /// Each continuous block of integers represents the
    /// items carried by one Elf. Blocks are separated by
    /// a blank line.
    fn accumulate(input: &str) -> Vec<Elf> {
        input
            .split("\n\n") // Split between blocks
            .map(|values| {
                values
                    .split("\n") // Split between items
                    .map(|calories| calories.parse::<u64>().unwrap())
                    .fold(0u64, |a, b| a + b)
            })
            .map(|calories| Elf(calories))
            .collect()
    }

    fn sum_largest_n(n: usize, mut elves: Vec<Elf>) -> Elf {
        elves.sort(); // Sort ascending
        elves.reverse(); // Sort descending
        elves.into_iter().take(n).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge_1() {
        let input = include_str!("input").trim();
        let mut elves = Elf::accumulate(input);
        elves.sort();
        let max_value = elves.pop();
        assert!(max_value.is_some());
        let max_value = max_value.unwrap();
        assert_eq!(max_value.0, 71023);
    }

    #[test]
    fn challenge_2() {
        let input = include_str!("input").trim();
        let elves = Elf::accumulate(input);
        let largest_three_sum = Elf::sum_largest_n(3, elves);
        assert_eq!(largest_three_sum.0, 206289);
    }
}
