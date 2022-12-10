#[cfg(test)]
mod tests {
    use std::{i16, str::FromStr};

    enum Instruction {
        NoOp,
        AddX(i16),
    }

    impl FromStr for Instruction {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(match s.split_once(" ") {
                Some((_addx, value)) => {
                    Self::AddX(value.parse::<i16>().expect("Failed to parse number"))
                }
                None => Self::NoOp,
            })
        }
    }

    fn part1(input: &str) -> i16 {
        let mut last_x = 1;
        let total_cycles = input.lines().fold(0, |sum, line| {
            sum + match line.parse::<Instruction>().ok().unwrap() {
                Instruction::NoOp => 1,
                Instruction::AddX(_) => 2,
            }
        });
        let evaluation_cycles = (0..((total_cycles + 20) / 40))
            .map(|val| 20 + 40 * val)
            .collect::<Vec<_>>();
        let mut iter = input
            .lines()
            .scan((0, 1), |(cycle, x), line| {
                match line.parse::<Instruction>().ok()? {
                    Instruction::NoOp => *cycle += 1,
                    Instruction::AddX(dx) => {
                        *cycle += 2;
                        *x += dx;
                    }
                }
                Some((*cycle, *x))
            })
            .peekable();

        evaluation_cycles
            .iter()
            .map(|tap| {
                while let Some((_, x)) = iter.peek().filter(|(cycle, _)| cycle < tap) {
                    last_x = *x;
                    iter.next();
                }
                tap * last_x
            })
            .sum()
    }

    fn part2(input: &str) -> String {
        let mut last_x = 1;
        let mut iter = input
            .lines()
            .scan((0, 1), |(cycle, x), line| {
                match line.parse::<Instruction>().ok()? {
                    Instruction::NoOp => *cycle += 1,
                    Instruction::AddX(dx) => {
                        *cycle += 2;
                        *x += dx;
                    }
                }
                Some((*cycle, *x))
            })
            .peekable();

        let total_cycles = input.lines().fold(0, |sum, line| {
            sum + match line.parse::<Instruction>().ok().unwrap() {
                Instruction::NoOp => 1,
                Instruction::AddX(_) => 2,
            }
        });
        (0..total_cycles)
            .step_by(40)
            .map(|row| {
                (0..40)
                    .map(|col| {
                        while let Some((_, x)) =
                            iter.peek().filter(|(cycle, _)| cycle <= &(row + col))
                        {
                            last_x = *x;
                            iter.next();
                        }
                        if (-1..=1).contains(&(last_x - col)) {
                            '\u{2593}'
                        } else {
                            '\u{2591}'
                        }
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    fn day_10_test_1() {
        let input = include_str!("test").trim();

        let result = part1(input);
        assert_eq!(result, 13140);
    }

    #[test]
    fn day_10_challenge_1() {
        let input = include_str!("input").trim();
        let result = part1(input);
        assert_eq!(result, 15120);
    }

    #[test]
    fn day_10_test_2() {
        let input = include_str!("test").trim();
        let result = part2(input);
        println!("{result}");
    }

    #[test]
    fn day_10_challenge_2() {
        let input = include_str!("input").trim();
        let result = part2(input);
        println!("{result}");
    }
}
