use std::ops::AddAssign;

#[derive(Debug, Default)]
struct Tree<T>
where
    T: PartialEq,
{
    nodes: Vec<Node<T>>,
}

impl<T> Tree<T>
where
    T: Copy + PartialEq + AddAssign,
{
    fn insert(&mut self, node: Node<T>) {
        self.nodes.push(node);
    }

    fn parent(&self, index: usize) -> Option<usize> {
        self.nodes[index].parent
    }

    fn child(&self, current_index: usize, target_label: &str) -> Option<usize> {
        self.nodes[current_index]
            .children
            .iter()
            .find(|child_index| self.nodes[**child_index].label == target_label)
            .copied()
    }

    fn register_file(&mut self, index: usize, file_size: T) {
        self.nodes[index].size += file_size;
        if let Some(index) = self.nodes[index].parent {
            self.register_file(index, file_size);
        }
    }
}

#[derive(Debug)]
struct Node<T>
where
    T: PartialEq,
{
    idx: usize,
    label: String,
    size: T,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl<T> Node<T>
where
    T: PartialEq,
{
    fn new(idx: usize, label: &str, size: T) -> Self {
        Self {
            idx,
            label: label.to_string(),
            size,
            parent: None,
            children: vec![],
        }
    }
}

#[derive(Debug)]
enum Line<'a> {
    ChangeDir(&'a str),
    Dir(&'a str),
    File((&'a str, u32)),
}

fn parse_line(line: &'_ str) -> Option<Line<'_>> {
    match line.split_once(' ') {
        Some(("$", command)) => match command.split_once(' ') {
            Some(("cd", dir)) => Some(Line::ChangeDir(dir)),
            _ => None,
        },
        Some(("dir", name)) => Some(Line::Dir(name)),
        Some((size, name)) => Some(Line::File((
            name,
            size.parse::<u32>().expect("Failed to parse filesize"),
        ))),
        _ => unreachable!("Malformed Input"),
    }
}

fn process_input(input: &str) -> Tree<u32> {
    let mut file_system: Tree<u32> = Tree { nodes: vec![] };
    file_system.insert(Node::new(0, "/", 0u32));
    let mut node_index = 0;

    for line in input.lines() {
        match parse_line(line) {
            Some(Line::ChangeDir(dir)) => match dir {
                "/" => node_index = 0,
                ".." => {
                    match file_system.parent(node_index) {
                        Some(parent_index) => {
                            node_index = parent_index;
                        }
                        None => unreachable!("No parent registered..."),
                    };
                }
                dir => {
                    match file_system.child(node_index, dir) {
                        Some(child_index) => {
                            node_index = child_index;
                        }
                        None => unreachable!("No child registered..."),
                    };
                }
            },
            Some(Line::Dir(dir_name)) => {
                let new_node_index = file_system.nodes.len();
                let mut node = Node::new(new_node_index, dir_name, 0u32);
                node.parent = Some(node_index);
                file_system.insert(node);
                file_system.nodes[node_index].children.push(new_node_index);
            }
            Some(Line::File((_, size))) => {
                file_system.register_file(node_index, size);
            }
            None => {}
        }
    }
    file_system
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_7_test_1() {
        let input = include_str!("test").trim();

        let file_system = process_input(input);

        let result = file_system
            .nodes
            .iter()
            .filter(|node| node.size <= 100000)
            .fold(0u32, |a, b| a + b.size);

        assert_eq!(result, 95437u32);
    }

    #[test]
    fn day_7_challenge_1() {
        let input = include_str!("input").trim();

        let file_system = process_input(input);

        let result = file_system
            .nodes
            .iter()
            .filter(|node| node.size <= 100000)
            .fold(0u32, |a, b| a + b.size);

        assert_eq!(result, 1453349u32);
    }

    #[test]
    fn day_7_test_2() {
        let input = include_str!("test").trim();

        let file_system = process_input(input);

        let file_system_size = 70000000;
        let required_space = 30000000;
        let root_size = file_system.nodes[0].size;
        let deficit = required_space - (file_system_size - root_size);

        let mut possible_nodes = file_system
            .nodes
            .iter()
            .filter(|node| node.size >= deficit)
            .collect::<Vec<_>>();

        possible_nodes.sort_by(|a, b| b.size.cmp(&a.size));
        let result = possible_nodes.pop().unwrap().size;
        assert_eq!(result, 24933642u32);
    }

    #[test]
    fn day_7_challenge_2() {
        let input = include_str!("input").trim();

        let file_system = process_input(input);

        let file_system_size = 70000000;
        let required_space = 30000000;
        let root_size = file_system.nodes[0].size;
        let deficit = required_space - (file_system_size - root_size);
        let mut possible_nodes = file_system
            .nodes
            .into_iter()
            .filter(|node| node.size >= deficit)
            .collect::<Vec<_>>();
        possible_nodes.sort_by(|a, b| b.size.cmp(&a.size));
        let result = possible_nodes.pop().unwrap().size;
        assert_eq!(result, 2948823u32);
    }
}
