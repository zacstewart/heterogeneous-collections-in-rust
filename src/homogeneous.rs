use super::Address;
use std::collections::HashMap;

pub struct Bucket {
    nodes: HashMap<Address, Node>
}

pub struct Node {
    address: Address
}

impl Bucket {
    pub fn new() -> Bucket {
        Bucket {
            nodes: HashMap::new()
        }
    }

    pub fn get(&self, address: &Address) -> Option<&Node> {
        self.nodes.get(address)
    }

    pub fn insert(&mut self, node: Node) {
        self.nodes.insert(node.address, node);
    }
}

impl Node {
    pub fn new(address: Address) -> Node {
        Node {
            address: address
        }
    }

    pub fn send(&self, message: &str) {
        println!("Sending: {}", message);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_node() {
        let mut bucket = Bucket::new();
        let node = Node::new(0);
        bucket.insert(node);
    }

    #[test]
    fn get_node() {
        let mut bucket = Bucket::new();
        let node = Node::new(0);
        bucket.insert(node);
        let node_ptr = bucket.get(&0).unwrap();
        node_ptr.send("hi!")
    }
}
