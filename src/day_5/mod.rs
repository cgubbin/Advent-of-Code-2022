#[cfg(test)]
mod tests {

    #[derive(Debug)]
    struct State(Vec<Vec<char>>);

    impl State {
        fn new(n: usize) -> Self {
            Self(vec![vec![]; n])
        }
    }

    impl std::fmt::Display for State {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(
                f,
                "{}",
                self.0
                    .iter()
                    .filter_map(|column| column.last())
                    .copied()
                    .collect::<String>()
            )
        }
    }

    fn parse_initial_state(initial_state: &str) -> State {
        // Split off the column names
        match initial_state.rsplit_once('\n') {
            Some((initial_state, column_names)) => {
                // As all columns can be empty we get the number of columns from the labels
                let number_of_columns = column_names.split_whitespace().count();
                let mut state = State::new(number_of_columns);

                initial_state.rsplit('\n').for_each(|row| {
                    let mut index = 0;
                    while let Some(substring) = row.get(index..index + 3) {
                        let mut chars = substring.chars();
                        if let Some('[') = chars.next() {
                            if let Some(ch) = chars.next() {
                                state.0[index / 4].push(ch);
                            }
                        }
                        index += 4;
                    }
                });
                state
            }
            _ => unreachable!("Malformed input"),
        }
    }

    fn parse_operation(operation: &str) -> [usize; 3] {
        operation
            .splitn(6, ' ')
            .skip(1)
            .step_by(2)
            .map(|num| num.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()
            .expect("Failed to parse operation to usize")
            .try_into()
            .expect("Expected exactly 3 elements")
    }

    fn apply_operations_1(state: &mut State, operations: &str) {
        operations.split('\n').for_each(|operation| {
            let operation = parse_operation(operation);
            for _ in 0..operation[0] {
                match state.0[operation[1] - 1].pop() {
                    Some(cr) => state.0[operation[2] - 1].push(cr),
                    None => continue,
                }
            }
        });
    }

    fn apply_operations_2(state: &mut State, operations: &str) {
        operations.split('\n').for_each(|operation| {
            let operation = parse_operation(operation);

            let split_index = state.0[operation[1] - 1].len() - operation[0];

            let mut popped = state.0[operation[1] - 1].split_off(split_index);

            state.0[operation[2] - 1].append(&mut popped);
        });
    }

    #[test]
    fn day_5_challenge_1() {
        let input = include_str!("input");

        let output = match input.split_once("\n\n") {
            Some((initial_state, operations)) => {
                let mut state = parse_initial_state(initial_state);

                let operations = operations.trim();
                apply_operations_1(&mut state, operations);
                state.to_string()
            }
            _ => unreachable!("Malformed input"),
        };

        assert_eq!(output, "HNSNMTLHQ");
    }

    #[test]
    fn day_5_challenge_2() {
        let input = include_str!("input");

        let output = match input.split_once("\n\n") {
            Some((initial_state, operations)) => {
                let mut state = parse_initial_state(initial_state);

                let operations = operations.trim();
                apply_operations_2(&mut state, operations);
                state.to_string()
            }
            _ => unreachable!("Malformed input"),
        };

        assert_eq!(output, "RNLFDJMCT");
    }
}
