# Heterogeneous Collections in Rust

---

## Homogeneous Collections

---

```rust
pub type Address = u16;

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

### Homogenous Collections
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

### Generics

- More flexible than homogeneous collection, but may be _too_ flexible:

    ```rust
    bucket.insert("I'm not a node".to_string(), 0)
    ```

- Can't use any functions of the node structs since compiler can't know what type they are, other than `N`
- A bucket can contain _only one_ of any kind of struct

^ Bucket can contain ANY kind of object. There is nothing enforcing what N is.

^ We have to pass in the address explicitly, rather than access it from `node.address` like before. This is because the compiler can't know what fields type N has.

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

### Trait-Bounded Generics

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

---

## Status vs. Dynamic Dispatch

---

```rust
use std::fmt::Display;

trait Fancy: Display {
    fn fancy(&self) -> String;
}

impl Fancy for u8 {
    fn fancy(&self) -> String {
        format!("💙 {} 💙", self)
    }
}

impl Fancy for u16 {
    fn fancy(&self) -> String {
        format!("💚 {} 💚", self)
    }
}
```

---

### Static Dispatch

```rust
fn print_fancy<T: Fancy>(thing: T) {
    println!("Static Print Fancy: {}", thing);
}

let number = 10u8;
print_fancy(number);

let number = 10u16;
print_fancy(number);
```

---

### Static Dispatch

```rust
fn __print_fancy_for_u8(thing: u8) {
    println!("Static Print Fancy: {}", thing);
}

fn __print_fancy_for_u16(thing: u16) {
    println!("Static Print Fancy: {}", thing);
}

let number = 10u8;
__print_fancy_for_u8(number);

let number = 10u16;
__print_fancy_for_u16(number);
```

---

### Static Dispatch

- Code path established at compile-time
- Generally more efficient
- Can lead to code "bloat" due to specific function for each type
- Aggressive compiler "optimization" can actually lead to slower performance by bloating instruction cache
- Rust stdlib prefers static-dispatch when possible

---

### Dynamic Dispatch

```rust
fn print_fancy(thing: &Fancy) {
    println!("Dynamic Print Fancy: {}", thing);
}

let trait_object = &10u8 as &Fancy;
print_fancy(trait_object);

let number = 10u16;
print_fancy(&number);
```

---

### Dynamic Dispatch is Provided by Trait Objects

Trait Objects can be obtained by casting

```rust
let trait_object = &10u8 as &Fancy;
print_fancy(trait_object);
```

...or coercing

```rust
let number = 10u16;
print_fancy(&number);
```

^ AKA type erasure, because we're erasing the compilers knowledge about the specific type of the pointer

---

### Why Pointers?


```
// 8 bits
let num:   [x|x|x|x|x|x|x|x] = 5;
print_fancy(_|_|_|_|_|_|_|_)

// 16 bits
let num:   [x|x|x|x|x|x|x|x|x|x|x|x|x|x|x|x] = 5;
print_fancy(_|_|_|_|_|_|_|_) // can't fit :(
```

^ `u8` and `u16` take up different amounts of space on the stack. Same function couldn't take raw values for each.

---

### Why Pointers?

```
// 8 bits
let num:   [x|x|x|x|x|x|x|x] = 5;
let ptr:   [size] = &num;
print_fancy(size)

// 16 bits
let num =  [x|x|x|x|x|x|x|x|x|x|x|x|x|x|x|x];
let ptr:   [size] = &num;
print_fancy(size) // ptrs are all the same size :)
```

---

### Trait Objects can be other kinds of pointers, too

```rust
let boxed = Box::new(10u8) as Box<Fancy>;
let shared_reference = &10u8 as &Fancy;
let mut_reference = &mut 10u8 as &mut Fancy;
let raw_pointer = &10u8 as *const Fancy;
let mut_raw_pointer = &mut 10u8 as *mut Fancy;
```

---


### Object Safety

```rust
let numbers = vec![1, 2, 3];
let trait_object = &numbers as &Clone;
```

```
--> src/dispatch.rs:61:40
   |
61 |         let trait_object = &numbers as &Clone;
   |                                        ^^^^^^ the trait `std::clone::Clone` cannot be made into an object
   |
   = note: the trait cannot require that `Self : Sized`
```

^ In general, if methods in the trait use Self, the trait cannot be object safe

---

# [fit] ❤
