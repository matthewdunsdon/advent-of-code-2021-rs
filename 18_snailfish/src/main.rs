use std::{
    cmp,
    io::{BufRead, BufReader},
    ops::Add,
    ptr,
    str::FromStr,
};

#[derive(Debug, PartialEq)]
enum NodeKind {
    Container(Box<Node>, Box<Node>),
    Value(u8),
}

#[derive(Debug, PartialEq)]
struct NodeVisit<'a> {
    node: &'a Node,
    depth: usize,
}
struct NodeVisitor<'a> {
    stack: Vec<NodeVisit<'a>>,
}

impl<'a> Iterator for NodeVisitor<'a> {
    type Item = NodeVisit<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let node_visit = self.stack.pop()?;

        if let NodeKind::Container(n1, n2) = &node_visit.node.kind {
            self.stack.push(NodeVisit {
                node: n2,
                depth: node_visit.depth + 1,
            });
            self.stack.push(NodeVisit {
                node: n1,
                depth: node_visit.depth + 1,
            });
        }

        Some(node_visit)
    }
}

#[derive(Debug, PartialEq)]
struct Node {
    kind: NodeKind,
}

impl Node {
    fn value_of(value: u8) -> Node {
        Node {
            kind: NodeKind::Value(value),
        }
    }

    fn containing(node1: Node, node2: Node) -> Node {
        Node {
            kind: NodeKind::Container(Box::new(node1), Box::new(node2)),
        }
    }

    fn magnitude(&self) -> u64 {
        match &self.kind {
            NodeKind::Container(left, right) => &left.magnitude() * 3 + &right.magnitude() * 2,
            NodeKind::Value(l) => u64::from(*l),
        }
    }

    fn visitor_iter(&self) -> NodeVisitor {
        NodeVisitor {
            stack: vec![NodeVisit {
                node: self,
                depth: 0,
            }],
        }
    }

    fn reduce(self) -> Self {
        let mut tree = self;
        let mut no_updates = false;

        while !no_updates {
            no_updates = true;
            while let Some(next_tree) = tree.update_if_explosion() {
                tree = next_tree;
                no_updates = false;
            }
            if let Some(next_tree) = tree.update_if_split() {
                tree = next_tree;
                no_updates = false;
            }
        }
        tree
    }

    fn update_if_explosion(&self) -> Option<Self> {
        let (explode_index, explode_visit, val1, val2) =
            self.visitor_iter().enumerate().find_map(|(i, entry)| {
                if entry.depth > 3 {
                    if let NodeKind::Container(n1, n2) = &entry.node.kind {
                        if let NodeKind::Value(val1) = n1.kind {
                            if let NodeKind::Value(val2) = n2.kind {
                                return Some((i, entry, val1, val2));
                            }
                        }
                    }
                }
                None
            })?;

        let previous_visit = self
            .visitor_iter()
            .enumerate()
            .filter(|(index, entry)| {
                *index < explode_index && matches!(entry.node.kind, NodeKind::Value(_))
            })
            .last()
            .map(|r| r.1);
        let next_visit = self
            .visitor_iter()
            .enumerate()
            .find(|(index, entry)| {
                *index > explode_index + 2 && matches!(entry.node.kind, NodeKind::Value(_))
            })
            .map(|r| r.1);

        Some(self.apply_explosion(&explode_visit, &previous_visit, &next_visit, val1, val2))
    }

    fn apply_explosion(
        &self,
        explode_visit: &NodeVisit,
        previous_visit: &Option<NodeVisit>,
        next_visit: &Option<NodeVisit>,
        val1: u8,
        val2: u8,
    ) -> Self {
        if ptr::eq(self, explode_visit.node) {
            return Node::value_of(0);
        }
        if let Some(previous_node) = previous_visit {
            if ptr::eq(self, previous_node.node) {
                if let NodeKind::Value(prev_val) = previous_node.node.kind {
                    return Node::value_of(prev_val + val1);
                }
            }
        }
        if let Some(next_node) = next_visit {
            if ptr::eq(self, next_node.node) {
                if let NodeKind::Value(next_val) = next_node.node.kind {
                    return Node::value_of(next_val + val2);
                }
            }
        }
        match &self.kind {
            NodeKind::Container(n1, n2) => Node::containing(
                n1.apply_explosion(explode_visit, previous_visit, next_visit, val1, val2),
                n2.apply_explosion(explode_visit, previous_visit, next_visit, val1, val2),
            ),
            NodeKind::Value(v) => Node::value_of(v.clone()),
        }
    }

    fn update_if_split(&self) -> Option<Self> {
        let split_visit = self.visitor_iter().find(|entry| {
            if let NodeKind::Value(val1) = &entry.node.kind {
                *val1 > 9
            } else {
                false
            }
        })?;

        Some(self.apply_split(&split_visit))
    }

    fn apply_split(&self, split_visit: &NodeVisit) -> Self {
        match &self.kind {
            NodeKind::Container(n1, n2) => {
                Node::containing(n1.apply_split(split_visit), n2.apply_split(split_visit))
            }
            NodeKind::Value(v) => {
                if ptr::eq(self, split_visit.node) {
                    return Node::containing(
                        Node::value_of(v.div_euclid(2)),
                        Node::value_of(v.add(1).div_euclid(2)),
                    );
                } else {
                    Node::value_of(v.clone())
                }
            }
        }
    }
}
impl Add for Node {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Node::containing(self, other).reduce()
    }
}

impl FromStr for Node {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(parse_tree(s)?.0)
    }
}

fn parse_tree(s: &str) -> Result<(Node, usize), &'static str> {
    let first_char = s.chars().nth(0).ok_or("Empty")?;
    let first_char = first_char as u8;
    match first_char {
        b'0'..=b'9' => {
            let value = first_char - b'0';
            let next_char = s.chars().nth(1).unwrap_or(']') as u8;
            if matches!(next_char, b'0'..=b'9') {
                Ok((Node::value_of(value * 10 + (next_char - b'0')), 2))
            } else {
                Ok((Node::value_of(value), 1))
            }
        }
        b'[' => {
            let (part1, index1) = parse_tree(&s[1..])?;
            let (part2, index2) = parse_tree(&s[(index1 + 2)..])?;
            Ok((Node::containing(part1, part2), index1 + index2 + 3))
        }
        _ => Err("Unrecognised input"),
    }
}

fn main() {
    let lines: Vec<String> = BufReader::new(std::io::stdin())
        .lines()
        .filter_map(|f| f.ok())
        .collect();

    let total_result = lines
        .clone()
        .iter()
        .filter_map(|s| s.parse::<Node>().ok())
        .fold(None, |current, next_node| match current {
            Some(prev_node) => Some(prev_node + next_node),
            None => Some(next_node),
        })
        .unwrap();

    println!("Total magnitude: {}", total_result.magnitude());

    let mut best_magnitude = 0;

    for line in lines.iter() {
        for other_line in lines.iter() {
            if line != other_line {
                let mag = line
                    .parse::<Node>()
                    .unwrap()
                    .add(other_line.parse::<Node>().unwrap())
                    .magnitude();
                best_magnitude = cmp::max(best_magnitude, mag);
            }
        }
    }
    println!("best_magnitude: {}", best_magnitude);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn check_tree_parse_simple() {
        let node = "[1,2]".parse::<Node>().unwrap();
        assert_eq!(node, Node::containing(Node::value_of(1), Node::value_of(2)));

        let mut iter = node.visitor_iter();

        assert_eq!(
            iter.next(),
            Some(NodeVisit {
                node: &Node::containing(Node::value_of(1), Node::value_of(2)),
                depth: 0
            })
        );
        assert_eq!(
            iter.next(),
            Some(NodeVisit {
                node: &Node::value_of(1),
                depth: 1
            })
        );
        assert_eq!(
            iter.next(),
            Some(NodeVisit {
                node: &Node::value_of(2),
                depth: 1
            })
        );
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn check_tree_parse_level() {
        let node = "[[1,2],3]".parse::<Node>().unwrap();
        assert_eq!(
            node,
            Node::containing(
                Node::containing(Node::value_of(1), Node::value_of(2)),
                Node::value_of(3)
            )
        );

        let mut iter = node.visitor_iter();

        assert_eq!(
            iter.next(),
            Some(NodeVisit {
                node: &Node::containing(
                    Node::containing(Node::value_of(1), Node::value_of(2)),
                    Node::value_of(3)
                ),
                depth: 0
            })
        );
        assert_eq!(
            iter.next(),
            Some(NodeVisit {
                node: &Node::containing(Node::value_of(1), Node::value_of(2)),
                depth: 1
            })
        );
        assert_eq!(
            iter.next(),
            Some(NodeVisit {
                node: &Node::value_of(1),
                depth: 2
            })
        );
        assert_eq!(
            iter.next(),
            Some(NodeVisit {
                node: &Node::value_of(2),
                depth: 2
            })
        );
        assert_eq!(
            iter.next(),
            Some(NodeVisit {
                node: &Node::value_of(3),
                depth: 1
            })
        );
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn check_tree_parse_multiple_levels() {
        let node = "[[1,2],[[3,4],5]]".parse::<Node>().unwrap();
        assert_eq!(
            node,
            Node::containing(
                Node::containing(Node::value_of(1), Node::value_of(2)),
                Node::containing(
                    Node::containing(Node::value_of(3), Node::value_of(4)),
                    Node::value_of(5)
                )
            )
        );

        let mut iter = node.visitor_iter();

        assert_eq!(
            iter.next(),
            Some(NodeVisit {
                node: &Node::containing(
                    Node::containing(Node::value_of(1), Node::value_of(2)),
                    Node::containing(
                        Node::containing(Node::value_of(3), Node::value_of(4)),
                        Node::value_of(5)
                    )
                ),
                depth: 0
            })
        );
        assert_eq!(
            iter.next(),
            Some(NodeVisit {
                node: &Node::containing(Node::value_of(1), Node::value_of(2)),
                depth: 1
            })
        );
        assert_eq!(
            iter.next(),
            Some(NodeVisit {
                node: &Node::value_of(1),
                depth: 2
            })
        );
        assert_eq!(
            iter.next(),
            Some(NodeVisit {
                node: &Node::value_of(2),
                depth: 2
            })
        );
        assert_eq!(
            iter.next(),
            Some(NodeVisit {
                node: &Node::containing(
                    Node::containing(Node::value_of(3), Node::value_of(4)),
                    Node::value_of(5)
                ),
                depth: 1
            })
        );
        assert_eq!(
            iter.next(),
            Some(NodeVisit {
                node: &Node::containing(Node::value_of(3), Node::value_of(4)),
                depth: 2
            })
        );
        assert_eq!(
            iter.next(),
            Some(NodeVisit {
                node: &Node::value_of(3),
                depth: 3
            })
        );
        assert_eq!(
            iter.next(),
            Some(NodeVisit {
                node: &Node::value_of(4),
                depth: 3
            })
        );
        assert_eq!(
            iter.next(),
            Some(NodeVisit {
                node: &Node::value_of(5),
                depth: 2
            })
        );
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn check_magnitude_simple_case() {
        assert_eq!(
            "[[1,2],[[3,4],5]]".parse::<Node>().unwrap().magnitude(),
            143
        );
    }
    #[test]
    fn check_magnitude_additional_cases() {
        assert_eq!(
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"
                .parse::<Node>()
                .unwrap()
                .magnitude(),
            1384
        );
        assert_eq!(
            "[[[[1,1],[2,2]],[3,3]],[4,4]]"
                .parse::<Node>()
                .unwrap()
                .magnitude(),
            445
        );
        assert_eq!(
            "[[[[3,0],[5,3]],[4,4]],[5,5]]"
                .parse::<Node>()
                .unwrap()
                .magnitude(),
            791
        );
        assert_eq!(
            "[[[[5,0],[7,4]],[5,5]],[6,6]]"
                .parse::<Node>()
                .unwrap()
                .magnitude(),
            1137
        );
        assert_eq!(
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
                .parse::<Node>()
                .unwrap()
                .magnitude(),
            3488
        );
    }

    #[test]
    fn check_update_if_explosion_when_no_explosion() {
        assert_eq!(
            "[[1,2],[[1,2],3]]"
                .parse::<Node>()
                .unwrap()
                .update_if_explosion(),
            None
        );
    }

    #[test]
    fn check_update_if_explosion_when_far_left_explosion() {
        assert_eq!(
            "[[[[[9,8],1],2],1],[4,5]]"
                .parse::<Node>()
                .unwrap()
                .update_if_explosion(),
            Some("[[[[0,9],2],1],[4,5]]".parse::<Node>().unwrap())
        );
    }

    #[test]
    fn check_update_if_explosion_when_far_right_explosion() {
        assert_eq!(
            "[[3,2],[4,[5,[4,[3,2]]]]]"
                .parse::<Node>()
                .unwrap()
                .update_if_explosion(),
            Some("[[3,2],[4,[5,[7,0]]]]]".parse::<Node>().unwrap())
        );
    }

    #[test]
    fn check_update_if_explosion_when_explosion_in_middle() {
        assert_eq!(
            "[[6,[5,[4,[3,2]]]],[3,2]]"
                .parse::<Node>()
                .unwrap()
                .update_if_explosion(),
            Some("[[6,[5,[7,0]]],[5,2]]".parse::<Node>().unwrap())
        );
    }

    #[test]
    fn check_update_if_explosion_when_multiple_explosions_present() {
        assert_eq!(
            "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]"
                .parse::<Node>()
                .unwrap()
                .update_if_explosion(),
            Some("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]".parse::<Node>().unwrap())
        );

        assert_eq!(
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"
                .parse::<Node>()
                .unwrap()
                .update_if_explosion(),
            Some("[[3,[2,[8,0]]],[9,[5,[7,0]]]]".parse::<Node>().unwrap())
        );
    }

    #[test]
    fn check_update_if_split_no_split() {
        assert_eq!(
            "[[1,2],[[1,2],3]]"
                .parse::<Node>()
                .unwrap()
                .update_if_split(),
            None
        );
    }

    #[test]
    fn check_update_if_split_single_split() {
        assert_eq!(
            "[[1,2],[[1,10],3]]"
                .parse::<Node>()
                .unwrap()
                .update_if_split(),
            Some("[[1,2],[[1,[5,5]],3]]".parse::<Node>().unwrap())
        );
    }

    #[test]
    fn check_update_if_split_multiple_splits() {
        assert_eq!(
            "[[[[0,7],4],[15,[0,13]]],[1,1]]"
                .parse::<Node>()
                .unwrap()
                .update_if_split(),
            Some(
                "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]"
                    .parse::<Node>()
                    .unwrap()
            )
        );

        assert_eq!(
            "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]"
                .parse::<Node>()
                .unwrap()
                .update_if_split(),
            Some(
                "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]"
                    .parse::<Node>()
                    .unwrap()
            )
        );
    }

    #[test]
    fn check_add_simple() {
        assert_eq!(
            "[1,2]".parse::<Node>().unwrap() + "[[1,2],3]".parse::<Node>().unwrap(),
            "[[1,2],[[1,2],3]]".parse::<Node>().unwrap()
        );
    }

    #[test]
    fn check_add_with_far_left_explosion() {
        assert_eq!(
            "[[[[9,8],1],2],3]".parse::<Node>().unwrap() + "[4,5]".parse::<Node>().unwrap(),
            "[[[[0,9],2],3],[4,5]]".parse::<Node>().unwrap()
        );
    }

    #[test]
    fn check_add_with_far_right_explosion() {
        assert_eq!(
            "[8,7]".parse::<Node>().unwrap() + "[6,[5,[4,[3,2]]]]".parse::<Node>().unwrap(),
            "[[8,7],[6,[5,[7,0]]]]".parse::<Node>().unwrap()
        );
    }

    #[test]
    fn check_add_with_middle_explosion() {
        assert_eq!(
            "[6,[5,[4,[3,2]]]]".parse::<Node>().unwrap() + "[1,2]".parse::<Node>().unwrap(),
            "[[6,[5,[7,0]]],[3,2]]".parse::<Node>().unwrap()
        );
    }

    #[test]
    fn check_add_with_explosions_and_splits() {
        assert_eq!(
            "[[[[4,3],4],4],[7,[[8,4],9]]]".parse::<Node>().unwrap()
                + "[1,1]".parse::<Node>().unwrap(),
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".parse::<Node>().unwrap()
        );
    }

    #[test]
    fn check_add_repeated_simple() {
        assert_eq!(
            "[1,1]".parse::<Node>().unwrap()
                + "[2,2]".parse::<Node>().unwrap()
                + "[3,3]".parse::<Node>().unwrap()
                + "[4,4]".parse::<Node>().unwrap()
                + "[5,5]".parse::<Node>().unwrap()
                + "[6,6]".parse::<Node>().unwrap(),
            "[[[[5,0],[7,4]],[5,5]],[6,6]]".parse::<Node>().unwrap()
        );
    }

    #[test]
    fn check_add_repeated_complex() {
        assert_eq!(
            "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]"
                .parse::<Node>()
                .unwrap()
                + "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]".parse::<Node>().unwrap()
                + "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]"
                    .parse::<Node>()
                    .unwrap()
                + "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]"
                    .parse::<Node>()
                    .unwrap()
                + "[7,[5,[[3,8],[1,4]]]]".parse::<Node>().unwrap()
                + "[[2,[2,2]],[8,[8,1]]]".parse::<Node>().unwrap()
                + "[2,9]".parse::<Node>().unwrap()
                + "[1,[[[9,3],9],[[9,0],[0,7]]]]".parse::<Node>().unwrap()
                + "[[[5,[7,4]],7],1]".parse::<Node>().unwrap()
                + "[[[[4,2],2],6],[8,7]]".parse::<Node>().unwrap(),
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
                .parse::<Node>()
                .unwrap()
        );

        assert_eq!(
            "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]"
                .parse::<Node>()
                .unwrap()
                + "[[[5,[2,8]],4],[5,[[9,9],0]]]".parse::<Node>().unwrap()
                + "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]".parse::<Node>().unwrap()
                + "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]".parse::<Node>().unwrap()
                + "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]"
                    .parse::<Node>()
                    .unwrap()
                + "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]"
                    .parse::<Node>()
                    .unwrap()
                + "[[[[5,4],[7,7]],8],[[8,3],8]]".parse::<Node>().unwrap()
                + "[[9,3],[[9,9],[6,[4,9]]]]".parse::<Node>().unwrap()
                + "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]"
                    .parse::<Node>()
                    .unwrap()
                + "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"
                    .parse::<Node>()
                    .unwrap(),
            "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]"
                .parse::<Node>()
                .unwrap()
        );
    }
}
