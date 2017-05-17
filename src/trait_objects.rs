use super::Address;
use std::collections::HashMap;

pub struct Bucket {
    nodes: HashMap<Address, Box<Node>>
}

impl Bucket {
    pub fn new() -> Bucket {
        Bucket {
            nodes: HashMap::new()
        }
    }

    pub fn get(&self, address: &Address) -> Option<&Box<Node>> {
        self.nodes.get(address)
    }

    pub fn insert(&mut self, node: Box<Node>) {
        self.nodes.insert(node.address(), node);
    }
}

pub struct UdpNode {
    address: Address
}

pub struct BluetoothNode {
    address: Address
}

pub trait Node {
    fn address(&self) -> Address;
    fn send(&self, message: &str);
}

impl UdpNode {
    pub fn new(address: Address) -> UdpNode {
        UdpNode {
            address: address
        }
    }
}

impl BluetoothNode {
    pub fn new(address: Address) -> BluetoothNode {
        BluetoothNode {
            address: address
        }
    }
}

impl Node for UdpNode {
    fn address(&self) -> Address {
        self.address
    }

    fn send(&self, message: &str) {
        println!("Sending via UDP: {}", message);
    }
}

impl Node for BluetoothNode {
    fn address(&self) -> Address {
        self.address
    }

    fn send(&self, message: &str) {
        println!("Sending via Bluetooth: {}", message);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_node() {
        let mut bucket = Bucket::new();

        let udp_node = UdpNode::new(0);
        let bluetooth_node = BluetoothNode::new(1);

        bucket.insert(Box::new(udp_node));
        bucket.insert(Box::new(bluetooth_node));
    }

    #[test]
    fn get_node() {
        let mut bucket = Bucket::new();

        let udp_node = UdpNode::new(0);
        let bluetooth_node = BluetoothNode::new(1);

        bucket.insert(Box::new(udp_node));
        bucket.insert(Box::new(bluetooth_node));

        bucket.get(&0).unwrap().send("Sending with UDP node");
        bucket.get(&1).unwrap().send("Sending with Bluetooth node");
    }
}
