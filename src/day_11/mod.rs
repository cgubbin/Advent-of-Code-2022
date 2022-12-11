#[cfg(test)]
mod tests {
    use std::{
        collections::{BTreeMap, VecDeque},
        str::FromStr,
    };

    #[derive(Clone, Copy, Debug)]
    enum Operation {
        Add(u64),
        Mul(u64),
        Square,
    }

    impl Default for Operation {
        fn default() -> Self {
            Self::Add(0)
        }
    }

    #[derive(Debug, Default)]
    struct Monkey {
        items: VecDeque<u64>,
        divisor: u64,
        if_true: usize,
        if_false: usize,
        operation: Operation,
        num_inspections: usize,
    }

    fn test_worry(worry_level: u64, divisor: u64) -> bool {
        (worry_level % divisor) == 0
    }

    fn operate(worry_level: u64, operation: Operation) -> u64 {
        match operation {
            Operation::Square => worry_level * worry_level,
            Operation::Add(x) => worry_level + x,
            Operation::Mul(x) => worry_level * x,
        }
    }

    fn operate_re(worry_level: u64, operation: Operation, p: u64) -> u64 {
        match operation {
            Operation::Square => ((worry_level % p) * (worry_level % p)) % p,
            Operation::Add(x) => ((worry_level % p) + (x % p)) % p,
            Operation::Mul(x) => ((worry_level % p) * (x % p)) % p,
        }
    }

    impl FromStr for Monkey {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut monkey = Monkey::default();
            s.lines()
                .skip(1)
                .for_each(|line| match line.trim().split_once(":") {
                    Some((label, items)) if label == "Starting items" => {
                        monkey.items = items
                            .split(",")
                            .map(|item| item.trim().parse::<u64>().unwrap())
                            .collect();
                    }
                    Some((label, statement)) if label == "Test" => {
                        monkey.divisor = statement
                            .rsplit_once(" ")
                            .unwrap()
                            .1
                            .parse::<u64>()
                            .unwrap();
                    }
                    Some((label, items)) if label == "If true" => {
                        monkey.if_true = items.rsplit_once(" ").unwrap().1.parse::<usize>().unwrap()
                    }
                    Some((label, items)) if label == "If false" => {
                        monkey.if_false =
                            items.rsplit_once(" ").unwrap().1.parse::<usize>().unwrap()
                    }
                    Some((label, items)) if label == "Operation" => {
                        let substrings = items.rsplitn(3, " ").collect::<Vec<_>>();
                        monkey.operation = match substrings[0] {
                            x if x == "old" => Operation::Square,
                            number => {
                                let number = number.parse::<u64>().unwrap();
                                match substrings[1].trim() {
                                    "+" => Operation::Add(number),
                                    "*" => Operation::Mul(number),
                                    _ => unreachable!("Malformed Input"),
                                }
                            }
                        };
                    }
                    _ => {}
                });
            Ok(monkey)
        }
    }

    fn parse(input: &str) -> BTreeMap<usize, Monkey> {
        input
            .split("\n\n")
            .enumerate()
            .map(|(idx, monkey)| {
                (
                    idx,
                    monkey.parse::<Monkey>().expect("Failed to create monkey"),
                )
            })
            .collect()
    }

    fn play_round(monkeys: &mut BTreeMap<usize, Monkey>, worry_unstable: bool) {
        let re_mod: u64 = if worry_unstable {
            // if x is divisible by d, it's also divisible by d * e. So we just use modulo product of all divisors. What's more, if x % d = y, then x % d * e = y as well.
            monkeys.values().map(|m| m.divisor).product()
        } else {
            1
        };

        let keys = monkeys.keys().copied().collect::<Vec<_>>();
        for idx in keys {
            // Remove so the monkey can be mutated while the rest of the
            // map is also mutated
            let mut monkey = monkeys.remove(&idx).unwrap();

            monkey.items.drain(..).for_each(|worry_level| {
                let worry_level = if !worry_unstable {
                    operate(worry_level, monkey.operation) / 3
                } else {
                    operate_re(worry_level, monkey.operation, re_mod)
                };
                let throw_index = match test_worry(worry_level, monkey.divisor) {
                    true => monkey.if_true,
                    false => monkey.if_false,
                };

                monkeys
                    .entry(throw_index)
                    .and_modify(|monkey| monkey.items.push_back(worry_level));
                monkey.num_inspections += 1;
            });

            monkeys.insert(idx, monkey);
        }
    }

    fn play(num_rounds: usize, monkeys: &mut BTreeMap<usize, Monkey>, worry_unstable: bool) {
        for _ in 0..num_rounds {
            play_round(monkeys, worry_unstable);
        }
    }

    #[test]
    fn day_11_test_1() {
        let input = include_str!("test").trim();
        let mut monkeys = parse(input);
        play(20, &mut monkeys, false);
        let mut num_inspections = monkeys
            .into_values()
            .map(|monkey| monkey.num_inspections)
            .collect::<Vec<_>>();
        num_inspections.sort();
        let result: usize = num_inspections.into_iter().rev().take(2).product();

        assert_eq!(result, 10605);
    }

    #[test]
    fn day_11_challenge_1() {
        let input = include_str!("input").trim();
        let mut monkeys = parse(input);
        play(20, &mut monkeys, false);
        let mut num_inspections = monkeys
            .into_values()
            .map(|monkey| monkey.num_inspections)
            .collect::<Vec<_>>();
        num_inspections.sort();
        let result: usize = num_inspections.into_iter().rev().take(2).product();

        assert_eq!(result, 90294);
    }

    #[test]
    fn day_11_test_2() {
        let input = include_str!("test").trim();
        let mut monkeys = parse(input);
        play(10000, &mut monkeys, true);
        let mut num_inspections = monkeys
            .into_values()
            .map(|monkey| monkey.num_inspections)
            .collect::<Vec<_>>();
        num_inspections.sort();
        let result: usize = num_inspections.into_iter().rev().take(2).product();

        assert_eq!(result, 2713310158);
    }

    #[test]
    fn day_11_challenge_2() {
        let input = include_str!("input").trim();
        let mut monkeys = parse(input);
        play(10000, &mut monkeys, true);
        let mut num_inspections = monkeys
            .into_values()
            .map(|monkey| monkey.num_inspections)
            .collect::<Vec<_>>();
        num_inspections.sort();
        let result: usize = num_inspections.into_iter().rev().take(2).product();

        assert_eq!(result, 18170818354);
    }
}
