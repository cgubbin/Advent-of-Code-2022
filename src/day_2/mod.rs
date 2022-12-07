use std::cmp::Ordering;

#[derive(Clone, PartialEq, Eq)]
enum Guess {
    Rock,
    Paper,
    Scissors,
}

impl Ord for Guess {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Guess::Rock, Guess::Rock) => Ordering::Equal,
            (Guess::Rock, Guess::Paper) => Ordering::Less,
            (Guess::Rock, Guess::Scissors) => Ordering::Greater,
            (Guess::Paper, Guess::Rock) => Ordering::Greater,
            (Guess::Paper, Guess::Paper) => Ordering::Equal,
            (Guess::Paper, Guess::Scissors) => Ordering::Less,
            (Guess::Scissors, Guess::Rock) => Ordering::Less,
            (Guess::Scissors, Guess::Paper) => Ordering::Greater,
            (Guess::Scissors, Guess::Scissors) => Ordering::Equal,
        }
    }
}

impl PartialOrd for Guess {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Guess {
    fn score(&self) -> usize {
        match *self {
            Guess::Rock => 1,
            Guess::Paper => 2,
            Guess::Scissors => 3,
        }
    }
}

struct Game {
    my_choice: Guess,
    opponent_choice: Guess,
}

impl Game {
    fn score_game(&self) -> usize {
        match self.my_choice.cmp(&self.opponent_choice) {
            Ordering::Greater => 6 + self.my_choice.score(),
            Ordering::Equal => 3 + self.my_choice.score(),
            Ordering::Less => self.my_choice.score(),
        }
    }
}

#[cfg(test)]
mod tests {

    fn parse_challenge_1(input: &str) -> Vec<Game> {
        input
            .split('\n')
            .map(|game| {
                let game = game.split(' ').collect::<Vec<_>>();
                assert_eq!(game.len(), 2);
                let opponent_choice = match game[0] {
                    "A" => Guess::Rock,
                    "B" => Guess::Paper,
                    "C" => Guess::Scissors,
                    _ => unreachable!(),
                };
                let my_choice = match game[1] {
                    "X" => Guess::Rock,
                    "Y" => Guess::Paper,
                    "Z" => Guess::Scissors,
                    _ => unreachable!(),
                };
                Game {
                    my_choice,
                    opponent_choice,
                }
            })
            .collect()
    }

    use super::*;

    #[test]
    fn challenge_1() {
        let input = include_str!("input").trim();
        let games = parse_challenge_1(input);
        let result = games.into_iter().fold(0, |a, b| a + b.score_game());
        assert_eq!(result, 9651);
    }

    fn parse_challenge_2(input: &str) -> Vec<Game> {
        input
            .split('\n')
            .map(|game| {
                let game = game.split(' ').collect::<Vec<_>>();
                assert_eq!(game.len(), 2);
                let opponent_choice = match game[0] {
                    "A" => Guess::Rock,
                    "B" => Guess::Paper,
                    "C" => Guess::Scissors,
                    _ => unreachable!(),
                };

                let my_choice = match game[1] {
                    "X" => match opponent_choice {
                        Guess::Rock => Guess::Scissors,
                        Guess::Paper => Guess::Rock,
                        Guess::Scissors => Guess::Paper,
                    },
                    "Y" => opponent_choice.clone(),
                    "Z" => match opponent_choice {
                        Guess::Rock => Guess::Paper,
                        Guess::Paper => Guess::Scissors,
                        Guess::Scissors => Guess::Rock,
                    },
                    _ => unreachable!(),
                };
                Game {
                    my_choice,
                    opponent_choice,
                }
            })
            .collect()
    }

    #[test]
    fn challenge_2() {
        let input = include_str!("input").trim();
        let games = parse_challenge_2(input);
        let result = games.into_iter().fold(0, |a, b| a + b.score_game());
        assert_eq!(result, 10560);
    }
}
