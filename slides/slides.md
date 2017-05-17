# Heterogeneous Collections in Rust

---

## Homogeneous Collections

---

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

Simple to implement, but limited to the one type (Node) it was coded for.

---

## Generics

---

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

---

### We can create a bucket that contains one type of node...

```rust
let mut udp_bucket = Bucket::new();
let udp_node = UdpNode::new(0);
udp_bucket.insert(udp_node, 0);
```

### Or the other

```rust
let mut bluetooth_bucket = Bucket::new();
let bluetooth_node = BluetoothNode::new(1);
bluetooth_bucket.insert(bluetooth_node, 1);
```
---

### But not both

```rust
let mut bucket = Bucket::new();

let udp_node = UdpNode::new(0);
let bluetooth_node = BluetoothNode::new(1);

bucket.insert(udp_node, 0);
bucket.insert(bluetooth_node, 1);
```

```
error[E0308]: mismatched types
   --> src/generics.rs:104:23
    |
104 |         bucket.insert(bluetooth_node, 1);
    |                       ^^^^^^^^^^^^^^ expected struct `generics::UdpNode`,
    |                                      found struct `generics::BluetoothNode`
    |
    = note: expected type `generics::UdpNode`
               found type `generics::BluetoothNode`
```

---

- More flexible than homogeneous collection, but may be _too_ flexible:

    ```rust
    bucket.insert("I'm not a node".to_string(), 0)
    ```

- Can't use any functions of the node structs since compiler can't know what type they are, other than `N`
- A bucket can contain _only one_ of any kind of struct

^ We have to pass in the address explicitly, rather than access it from `node.address` like before. This is because the compiler can't know what fields type N has.

^ Another consequence is that Bucket can contain ANY kind of object. There is nothing enforcing what N is.

^ We can improve upon that a little with bounds

---

## Generics Part 2: Trait Bounds

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

### Unbounded

```rust
pub struct Bucket<N> {
    nodes: HashMap<Address, N>
}

impl<N> Bucket<N> {
    pub fn get(&self, address: &Address) -> Option<&N> {
        self.nodes.get(address)
    }

    pub fn insert(&mut self, address: Address, node: N) {
        self.nodes.insert(address, node);
    }
}
```

^ Let's look at the non-bounded generic Bucket

---

### Bounded

```rust
pub struct Bucket<N: Node> {
    nodes: HashMap<Address, N>
}

impl<N: Node> Bucket<N> {
    pub fn get(&self, address: &Address) -> Option<&N> {
        self.nodes.get(address)
    }

    pub fn insert(&mut self, node: N) {
        self.nodes.insert(node.address(), node);
    }
}
```

^ Now, N must be something that implements Node, and since we know all things that implement Node had an address method that returns and Address, the compiler can safely verify that node.address() can be used in insert.

---


### We can create a bucket that contains one type of node...

```rust
let mut udp_bucket = Bucket::new();
let udp_node = UdpNode::new(0);
udp_bucket.insert(udp_node);
```

### Or the other

```rust
let mut bluetooth_bucket = Bucket::new();
let bluetooth_node = BluetoothNode::new(1);
bluetooth_bucket.insert(bluetooth_node);
```

---

### But not both

```rust
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

---

- Controlled flexibility: buckets can only contain things that `impl Node`
- Can call methods that `trait Node` declares: `node.address()`
- A bucket can still only contain one type of thing that impements `Node`

_Heterogeneous potential, homogeneous usage_

---

## Trait Objects

---

### Bounded generic

```rust
pub struct Bucket<N: Node> {
    nodes: HashMap<Address, N>
}

impl<N: Node> Bucket<N> {
    pub fn get(&self, address: &Address) -> Option<&N> {
        self.nodes.get(address)
    }

    pub fn insert(&mut self, node: N) {
        self.nodes.insert(node.address(), node);
    }
}
```

^ Let's go back to our generic Bucket for a sec
^ Let's modify the bucket to store Node trait objects.
^ We'll get rid of type parameter declarations for N
^ We'll replace usage of N with Box<Node> (boxed Node)

---

### Trait objects

```rust
pub struct Bucket {
    nodes: HashMap<Address, Box<Node>>
}

impl Bucket {
    pub fn get(&self, address: &Address) -> Option<&Box<Node>> {
        self.nodes.get(address)
    }

    pub fn insert(&mut self, node: Box<Node>) {
        self.nodes.insert(node.address(), node);
    }
}
```

^ Notice how all the Nodes are wrapped in a Box, which is just a pointer to memory on the heap.

---

```rust
impl UdpNode {
    pub fn send(&self, message: &str) {
        println!("Sending via UDP: {}", message);
    }
}

impl BluetoothNode {
    pub fn send(&self, message: &str) {
        println!("Sending via Bluetooth: {}", message);
    }
}
```

^ Now lets do the Node structs
^ We did have specific implementation for different kinds of nodes. If we want to use them as trait objects, we can only use the interface they share as Nodes.

---

```rust
pub trait Node {
    fn address(&self) -> Address;
    fn send(&self, message: &str); // <- move send declaration to here
}

impl Node for UdpNode {
    fn address(&self) -> Address {
        self.address
    }

    fn send(&self, message: &str) {
        println!("Sending via UDP: {}", message);
    }
}

// Same for BluetoothNode
```

^ We have to move the send function into the Node trait

---

### We can put both kinds of nodes in the same bucket

```rust
let mut bucket = Bucket::new();

let udp_node = UdpNode::new(0);
let bluetooth_node = BluetoothNode::new(1);

bucket.insert(Box::new(udp_node));
bucket.insert(Box::new(bluetooth_node));
```

### We can borrow and use both kinds because they `impl Node`

```rust
bucket.get(&0).unwrap().send("Sending with UDP node");
bucket.get(&1).unwrap().send("Sending with Bluetooth node");
```

---

- Buckets can only contain things that `impl Node`
- Can call methods that `trait Node` declares
- One bucket can store any mixture of things that `imple Node`
- Anything borrowed _from_ the bucket must be treated as a `Node`. Compiler can't know what kind of object was actually borrowed, just the trait.
