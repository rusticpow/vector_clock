use std::{cmp::max, ops::Deref};

fn main() {}

struct VectorClock {}

impl VectorClock {
    fn add_internal_event(nodes: &mut Vec<Node>, node_id: u32) {
        let node_result = nodes.iter_mut().find(|r| r.id == node_id);

        let mut node: &mut Node;
        if node_result.is_none() {
            let n = Node {
                id: node_id,
                timestamp: 1,
            };

            nodes.push(n);
        } else {
            node = node_result.unwrap();
            node.timestamp += 1;
        }
    }

    fn receive_event(
        current_nodes: &mut Vec<Node>,
        incoming_nodes: &mut Vec<Node>,
        node_id: u32,
    ) -> Result<(), ()> {
        let current_node = match current_nodes.iter_mut().find(|r| r.id == node_id) {
            Some(n) => n,
            None => return Err(()),
        };
        current_node.timestamp += 1;

        for in_node in incoming_nodes.iter_mut() {
            let node = match current_nodes.iter_mut().find(|r| r.id == in_node.id) {
                Some(n) => n,
                None => {
                    current_nodes.push(*in_node);
                    continue;
                }
            };

            node.timestamp = max(in_node.timestamp, node.timestamp);
        }

        Ok(())
    }
}

#[derive(Clone, Copy)]
struct Node {
    id: u32,
    timestamp: u64,
}

impl Node {}

#[cfg(test)]
mod tests {
    use crate::{Node, VectorClock};

    #[test]
    fn add_internal_event_no_nodes_add_new() {
        let mut nodes: Vec<Node> = vec![];

        VectorClock::add_internal_event(&mut nodes, 1);

        assert_eq!(1, nodes.len());
        assert_eq!(1, nodes[0].id);
        assert_eq!(1, nodes[0].timestamp);
    }

    #[test]
    fn add_internal_event_multiple_nodes_iterate_requested() {
        let mut nodes: Vec<Node> = vec![
            Node {
                id: 3,
                timestamp: 2,
            },
            Node {
                id: 2,
                timestamp: 3,
            },
            Node {
                id: 1,
                timestamp: 4,
            },
        ];

        VectorClock::add_internal_event(&mut nodes, 1);

        assert_eq!(3, nodes.len());
        assert_eq!(1, nodes[2].id);
        assert_eq!(2, nodes[0].timestamp);
        assert_eq!(3, nodes[1].timestamp);
        assert_eq!(5, nodes[2].timestamp);
    }

    #[test]
    fn receive_event_with_same_nodes() {
        let mut current_nodes: Vec<Node> = vec![
            Node {
                id: 3,
                timestamp: 2,
            },
            Node {
                id: 2,
                timestamp: 3,
            },
            Node {
                id: 1,
                timestamp: 4,
            },
        ];

        let mut incoming_nodes: Vec<Node> = vec![
            Node {
                id: 3,
                timestamp: 1,
            },
            Node {
                id: 2,
                timestamp: 5,
            },
            Node {
                id: 1,
                timestamp: 4,
            },
        ];

        VectorClock::receive_event(&mut current_nodes, &mut incoming_nodes, 1).unwrap();

        assert_eq!(3, current_nodes.len());
        assert_eq!(2, current_nodes[0].timestamp);
        assert_eq!(5, current_nodes[1].timestamp);
        assert_eq!(5, current_nodes[2].timestamp);
    }

    #[test]
    fn receive_event_when_node_no_in_current_nodes_add() {
        let mut current_nodes: Vec<Node> = vec![
            Node {
                id: 3,
                timestamp: 2,
            },
            Node {
                id: 2,
                timestamp: 3,
            },
            Node {
                id: 1,
                timestamp: 4,
            },
        ];

        let mut incoming_nodes: Vec<Node> = vec![
            Node {
                id: 3,
                timestamp: 1,
            },
            Node {
                id: 2,
                timestamp: 5,
            },
            Node {
                id: 1,
                timestamp: 4,
            },
            Node {
                id: 4,
                timestamp: 2,
            },
        ];

        VectorClock::receive_event(&mut current_nodes, &mut incoming_nodes, 1).unwrap();

        assert_eq!(4, current_nodes.len());
        assert_eq!(2, current_nodes[0].timestamp);
        assert_eq!(5, current_nodes[1].timestamp);
        assert_eq!(5, current_nodes[2].timestamp);
        assert_eq!(4, current_nodes[3].id);
        assert_eq!(2, current_nodes[3].timestamp);
    }
}
