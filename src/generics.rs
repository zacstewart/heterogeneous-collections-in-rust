use super::Address;
use std::collections::HashMap;

pub struct Bucket<N: Node> {
    nodes: HashMap<Address, N>
}

pub struct UdpNode {
    address: Address
}

pub struct BluetoothNode {
    address: Address
}

pub trait Node {
    fn address(&self) -> Address;
}

impl<N: Node> Bucket<N> {
    pub fn new() -> Bucket<N> {
        Bucket {
            nodes: HashMap::new()
        }
    }

    pub fn get(&self, address: &Address) -> Option<&N> {
        self.nodes.get(address)
    }

    pub fn insert(&mut self, node: N) {
        self.nodes.insert(node.address(), node);
    }
}

impl UdpNode {
    pub fn new(address: Address) -> Self {
        Self {
            address: address
        }
    }

    pub fn send(&self, message: &str) {
        println!("Sending via UDP: {}", message);
    }
}

impl Node for UdpNode {
    fn address(&self) -> Address {
        self.address
    }
}

impl Node for BluetoothNode {
    fn address(&self) -> Address {
        self.address
    }
}

impl BluetoothNode {
    pub fn new(address: Address) -> Self {
        Self {
            address: address
        }
    }

    pub fn send(&self, message: &str) {
        println!("Sending via Bluetooth: {}", message);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_node() {
        let mut udp_bucket = Bucket::new();
        let udp_node = UdpNode::new(0);
        udp_bucket.insert(udp_node);

        let mut bluetooth_bucket = Bucket::new();
        let bluetooth_node = BluetoothNode::new(1);
        bluetooth_bucket.insert(bluetooth_node);
    }

    #[test]
    fn get_node() {
        let mut udp_bucket = Bucket::new();
        let udp_node = UdpNode::new(0);
        udp_bucket.insert(udp_node);
        let node_ptr = udp_bucket.get(&0).unwrap();
        node_ptr.send("hi!");
    }

    #[test]
    fn insert_another_kind_of_node() {
        let mut bucket = Bucket::new();

        let udp_node = UdpNode::new(0);
        bucket.insert(udp_node);

        let bluetooth_node = BluetoothNode::new(0);
        bucket.insert(bluetooth_node);
    }
}
