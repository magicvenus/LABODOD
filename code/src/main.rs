pub struct Node<T> {
    _value: T,
    neighbors: List<T>,
}

pub struct List<T> {
    pub(crate) head: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            _value: value,
            neighbors: List { head: None },
        }
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push_back(&mut self, value: T) {
        let new_node = Box::new(Node {
            _value: value,
            neighbors: List { head: None },
        });

        if let Some(ref mut head) = self.head {
            let mut current = &mut head.neighbors.head;
            while let Some(ref mut next_node) = *current {
                current = &mut next_node.neighbors.head;
            }
            *current = Some(new_node);
        } else {
            self.head = Some(new_node);
        }
    }

    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut current = &self.head;
        while let Some(ref node) = *current {
            count += 1;
            current = &node.neighbors.head;
        }
        count
    }

    pub fn index(&self, index: usize) -> Option<&T> {
        let mut current = &self.head;
        let mut i = 0;
        while let Some(ref node) = *current {
            if i == index {
                return Some(&node._value);
            }
            i += 1;
            current = &node.neighbors.head;
        }
        None
    }
}

pub struct Graph<T> {
    nodes: List<T>,
}

impl<T> Graph<T> {
    pub fn new() -> Self {
        Graph { nodes: List::new() }
    }

    pub fn add_node(&mut self, value: T) {
        self.nodes.push_back(value);
    }

    pub fn contains(&self, value: T) -> bool
    where
        T: PartialEq,
    {
        let mut current = &self.nodes.head;
        while let Some(ref node) = *current {
            if node._value == value {
                return true;
            }
            current = &node.neighbors.head;
        }
        false
    }
}

fn main() {
    let mut graph = Graph::new();

    let a = 0;
    let b = 1;
    let c = 2;
    let d = 3;

    graph.add_node(a);
    graph.add_node(b);
    graph.add_node(c);
    graph.add_node(d);

    println!("Graph contains '2': {}", graph.contains(2));
    println!("Graph contains '5': {}", graph.contains(5));
}
