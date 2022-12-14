use regex::Regex;
use std::cmp::Ordering;
use std::error::Error;

/// A (possibly) nested list of integers.
///
#[derive(Debug, Clone, PartialEq, Eq)]
enum NestedList {
    Empty,
    Value(u16),
    List(Vec<NestedList>),
}

impl Ord for NestedList {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Empty, Self::Empty) => Ordering::Equal,
            (Self::Empty, _) => Ordering::Less,
            (_, Self::Empty) => Ordering::Greater,
            (Self::Value(v1), Self::Value(v2)) => v1.cmp(v2),
            (Self::Value(v), Self::List(li)) => vec![Self::Value(*v)].cmp(li),
            (Self::List(li), Self::Value(v)) => li.cmp(&vec![Self::Value(*v)]),
            (Self::List(li1), Self::List(li2)) => li1.cmp(li2),
        }
    }
}

impl PartialOrd for NestedList {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl NestedList {
    fn decode(input: &str) -> Result<Vec<Self>, Box<dyn Error>> {
        let tokens = Regex::new(r"[\[\]]|\d+")?;
        let mut lists = vec![];
        let mut li_stack = vec![];
        let mut li_curr = Self::Empty;

        macro_rules! invalid_input {
            ($msg:expr) => {
                return Err(format!("Invalid input - {}", $msg).into())
            };
        }

        for line in input.lines() {
            for token in tokens.captures_iter(&line) {
                match &token[0] {
                    "[" => {
                        let list = Self::List(vec![]);
                        match li_curr {
                            Self::Empty => {
                                li_curr = list;
                            }
                            Self::List(_) => {
                                li_stack.push(li_curr);
                                li_curr = list;
                            }
                            Self::Value(_) => {
                                panic!("wrong variant for `li_curr`");
                            }
                        }
                    }
                    "]" => match li_curr {
                        Self::List(_) => {
                            if let Some(mut nl) = li_stack.pop() {
                                if let Self::List(li) = &mut nl {
                                    li.push(li_curr);
                                    li_curr = nl;
                                } else {
                                    panic!("wrong variant on `li_stack`");
                                }
                            } else {
                                lists.push(li_curr);
                                li_curr = Self::Empty;
                            }
                        }
                        _ => {
                            panic!("wrong variant for `li_curr`");
                        }
                    },
                    num => match &mut li_curr {
                        Self::Empty => {
                            invalid_input!("no list for value");
                        }
                        Self::List(li) => {
                            li.push(Self::Value(num.parse::<u16>()?));
                        }
                        Self::Value(_) => {
                            invalid_input!("attempt to push value to value");
                        }
                    },
                }
            }
        }

        Ok(lists)
    }
}

#[cfg(test)]
mod tests {
    use super::NestedList;

    fn part_1(decoded: Vec<NestedList>) -> usize {
        decoded
            .chunks(2)
            .enumerate()
            .fold(0, |sum, (idx, chunk)| match chunk[0] < chunk[1] {
                true => sum + idx + 1,
                false => sum,
            })
    }

    fn part_2(mut decoded: Vec<NestedList>) -> usize {
        let packet_1 = NestedList::List(vec![NestedList::Value(2)]);
        let packet_2 = NestedList::List(vec![NestedList::Value(6)]);

        decoded.push(packet_1.clone());
        decoded.push(packet_2.clone());

        decoded.sort();

        let position_1 = decoded.iter().position(|x| x.clone() == packet_1).unwrap() + 1;
        let position_2 = decoded.iter().position(|x| x.clone() == packet_2).unwrap() + 1;

        position_1 * position_2
    }

    #[test]
    fn day_13_test_1() {
        let input = include_str!("test").trim();
        let decoded = NestedList::decode(input).expect("Decode failed");
        let result = part_1(decoded);
        assert_eq!(result, 13);
    }

    #[test]
    fn day_13_challenge_1() {
        let input = include_str!("input").trim();
        let decoded = NestedList::decode(input).expect("Decode failed");
        let result = part_1(decoded);
        assert_eq!(result, 5852);
    }

    #[test]
    fn day_13_test_2() {
        let input = include_str!("test").trim();
        let decoded = NestedList::decode(input).expect("Decode failed");
        let result = part_2(decoded);
        assert_eq!(result, 140);
    }

    #[test]
    fn day_13_challenge_2() {
        let input = include_str!("input").trim();
        let decoded = NestedList::decode(input).expect("Decode failed");
        let result = part_2(decoded);
        assert_eq!(result, 140);
    }
}
