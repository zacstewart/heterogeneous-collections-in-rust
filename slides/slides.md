# Heterogeneous Collections in Rust

---

## Homogeneous Collection

```rust
pub type Address = u64;

pub struct Bucket {
    nodes: HashMap<Address, Node>
}

pub struct Node {
    address: Address
}
```

^ Only one kind of Node

---

```rust
impl Bucket {
    pub fn get(&self, address: &Address) -> Option<&Node> {
        self.nodes.get(address)
    }

    pub fn insert(&mut self, node: Node) {
        self.nodes.insert(node.address, node);
    }
}
```

^ Basically just wrapping a HashMap

^ Accessing address field of Node to know where to insert it in the HashMap

---

```rust
// Put a node in the bucket...
let mut bucket = Bucket::new();
let node = Node::new(0);
bucket.insert(node);

// And borrow it back to use it...
let node_ptr = bucket.get(&0).unwrap();
node_ptr.send("hi!")
```

---

## Generics

```rust
pub struct Bucket<N> {
    nodes: HashMap<Address, N>
}

pub struct UdpNode {
    address: Address
}

pub struct BluetoothNode {
    address: Address
}
```

^ Two different kinds of nodes. A bucket can container one or the other (not both). Bucket<UdpNode> and Bucket<BluetoothNode>

---


```rust
impl<N> Bucket<N> {
    pub fn get(&self, address: &Address) -> Option<&N> {
        self.nodes.get(address)
    }

    pub fn insert(&mut self, address: Address, node: N) {
        self.nodes.insert(address, node);
    }
}
```

^ We have to pass in the address explicitly, rather than access it from `node.address` like before. This is because the compiler can't know what fields type N has.

^ Another consequence is that Bucket can contain ANY kind of object. There is nothing enforcing what N is.

^ We can improve upon that a little with bounds. We can say that all N must implement a trait.

---

```rust
pub trait Node {
    fn address(&self) -> Address;
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
```

---

```rust
pub struct Bucket<N: Node> {
    nodes: HashMap<Address, N>
}

impl<N: Node> Bucket<N> {
    pub fn insert(&mut self, node: N) {
        self.nodes.insert(node.address(), node);
    }
}
```

^ Now, N must be something that implements Node, and since we know all things that implement Node had an address method that returns and Address, the compiler can safely verify that node.address() can be used in insert.

---


```rust
// We can create a bucket that contains one type of node...
let mut udp_bucket = Bucket::new();
let udp_node = UdpNode::new(0);
udp_bucket.insert(udp_node);

// Or the other
let mut bluetooth_bucket = Bucket::new();
let bluetooth_node = BluetoothNode::new(1);
bluetooth_bucket.insert(bluetooth_node);
```

---

```rust
// But not both
let mut bucket = Bucket::new();

let udp_node = UdpNode::new(0);
let bluetooth_node = BluetoothNode::new(1);

bucket.insert(udp_node);
bucket.insert(bluetooth_node);
```

```
error[E0308]: mismatched types
   --> src/generics.rs:104:23
    |
104 |         bucket.insert(bluetooth_node);
    |                       ^^^^^^^^^^^^^^ expected struct `generics::UdpNode`,
    |                                      found struct `generics::BluetoothNode`
    |
    = note: expected type `generics::UdpNode`
               found type `generics::BluetoothNode`
```
