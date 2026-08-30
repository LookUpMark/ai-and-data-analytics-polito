# Smart Pointers

---

## Operations on Pointers

Every program value exists in the process's address space. The address-of operator `&` (and `&mut` in Rust) gets the memory address, subject to Rust's borrow checker. Dereferencing (`*`, `.` for member access) transforms an address into the value.

Rust and C++ allow operator overloading for user-defined types, enabling **"smart pointers."** These types syntactically resemble pointers but add characteristics like automatic memory management, reference counting, or controlled access. Rust smart pointers often *own* data, unlike references (`&T`, `&mut T`) which *borrow*.

---

## Use of Pointers

Pointers enable **dynamic data structures** (graphs, trees, lists). This flexibility risks null dereferences, memory leaks, data races.

Rust's borrow checker rules, applied to references (`&T`, `&mut T`), prevent direct **cyclic data structures**, ensuring a single ownership tree and enabling strong static analysis.

**Smart pointers** like `Rc<T>` (single-threaded) and `Arc<T>` (multi-threaded) allow **multiple owners**. `std::rc::Weak` and `std::sync::Weak` ("weak" pointers) enable cycles while preventing leaks from strong reference cycles.

---

## Smart Pointers in C++

C++ standard smart pointers:

*   **`std::unique_ptr<T>`:** Exclusive ownership of heap-allocated `T`. Memory auto-released on scope exit (destructor). Cannot be copied, only **moved**. Created with `std::make_unique<T>`. Internally, a raw pointer plus RAII logic and disabled copy/assignment. Custom deallocators possible.

*   **`std::shared_ptr<T>`:** Shared ownership of heap-allocated `T`. Uses a control block with atomic **strong reference count** and **weak reference count**. **Can be copied**, incrementing strong count. Data released when strong count reaches 0. Control block released when both counts are 0. Overhead: "fat pointer" (data ptr + control block ptr). Created with `std::make_shared<T>` (often more efficient). Custom deallocators possible. Problem: **cyclic graphs** of `shared_ptr` cause memory leaks as strong counts never reach zero.

*   **`std::weak_ptr<T>`:** Non-owning ("weak") reference to object managed by `shared_ptr`. Does **not** increment strong count, used to break cycles. Cannot be directly dereferenced; must be "locked" (`lock()`) to get a `shared_ptr`. `lock()` returns `Some(shared_ptr)` if data alive, `None` if deallocated. Created from `shared_ptr` via `std::shared_ptr::weak_from_this()` or constructor. Increments weak count.

---

## `unique_ptr<T>` in C++ Example

```cpp
#include <memory> // Required for smart pointers
#include <iostream>

void example_unique_ptr() {
    // p now owns a new integer on the heap, initialized to 5.
    std::unique_ptr<int> p = std::make_unique<int>(5);

    // Dereference p to get the value.
    int i = *p;
    std::cout << "Value of i: " << i << std::endl; // Output: 5

    // Modify the value on the heap through p.
    *p = 7;
    std::cout << "Value pointed to by p: " << *p << std::endl; // Output: 7

} // p goes out of scope here. Its destructor deallocates the integer 7 from the heap.
```

*(Diagram: `p` (stack) -> `int: 5` (heap). After `*p=7`, `p` (stack) -> `int: 7` (heap). On scope exit, heap `int` is deallocated.)*

---

## `shared_ptr<T>` in C++ Example

```cpp
#include <memory>
#include <iostream>

void example_shared_ptr() {
    std::shared_ptr<int> p = std::make_shared<int>(5);
    std::cout << "p use_count: " << p.use_count() << std::endl; // Output: 1

    { // Inner scope
        std::shared_ptr<int> q = p; // q is a copy of p
        std::cout << "p use_count: " << p.use_count() << ", q use_count: " << q.use_count() << std::endl; // Output: 2, 2

        *q = 3; // Modify the shared data through q
        std::cout << "Value via p: " << *p << ", Value via q: " << *q << std::endl; // Output: 3, 3
    } // q goes out of scope. Strong count decrements.
    std::cout << "p use_count after q is destroyed: " << p.use_count() << std::endl; // Output: 1

    *p = 7; // Modify the shared data through p
    std::cout << "Value via p: " << *p << std::endl; // Output: 7

} // p goes out of scope. Strong count decrements to 0. Data and control block deallocated.
```

*(Diagram: `p` (stack) -> Control Block (strong:1, weak:0, ptr) -> Data (int:5). `q = p` results in `p, q` (stack) -> same Control Block (strong:2, weak:0, ptr) -> same Data (int:5). On scope exit for `q`, strong count decrements. On scope exit for `p`, strong count becomes 0, leading to deallocation.)*

---

## Smart Pointers in Rust

Rust offers various smart pointers, often implemented as `struct`s holding a pointer and implementing `Deref`/`DerefMut` traits for pointer-like behavior.

*   `Box<T>`: Exclusive heap ownership (like C++ `unique_ptr`).
*   `Rc<T>`: Single-threaded shared ownership (like C++ `shared_ptr`, non-atomic).
*   `Arc<T>`: Multi-threaded shared ownership (like C++ `shared_ptr`, atomic).
*   `Weak<T>`: Non-owning references (`std::rc::Weak`, `std::sync::Weak`), for cycles.
*   `Cell<T>`, `RefCell<T>`: Interior mutability (runtime borrow checks), single-threaded.
*   `Cow<T>`: Clone-on-Write.
*   `Mutex<T>`, `RwLock<T>`: Concurrent synchronization.

---

## The `Deref` and `DerefMut` Traits in Rust

These traits allow a type to be treated like a reference (`&T` or `&mut T`).

*   **`trait Deref`**: `fn deref(&self) -> &Self::Target;` Allows immutable dereferencing (`*`, automatic for `.`).
*   **`trait DerefMut: Deref`**: `fn deref_mut(&mut self) -> &mut Self::Target;` Allows mutable dereferencing (`*`, automatic for `.`).

---

## `std::Box<T>` (Boxed Pointer)

*   `Box<T>` owns data **dynamically on the heap** (`Box::new(t)`).
*   **Exclusive Ownership:** Data is exclusively owned by the `Box`. Heap memory auto-released when `Box` drops (RAII via `Drop` trait). `drop(b)` forces early release.
*   **Move Semantics:** Moving `Box` transfers ownership of pointer/heap data. Lifetimes can extend.
*   **Unsized Types:** Can hold unsized types (`[T]`, `dyn Trait`). `Box<[T]>` or `Box<dyn Trait>` are "fat pointers" (data ptr + size/vtable ptr).
*   Essential for **recursive types** to break infinite size recursion, and for **trait objects** (`Box<dyn Trait>`) enabling dynamic dispatch.

---

## Recursive Types: Solution with `Box<T>`

Direct recursive enum `enum List { Cons(i32, List), Nil }` fails compile-time size check. `Box<T>` provides indirection.

```rust
enum List {
    Cons(i32, Box<List>), // Recursive List is heap-allocated via Box
    Nil,
}
```

`Box<List>` has a known, fixed size (size of a pointer), breaking the recursion for the compiler.

---

## `Box<T>` with Cons List Example (`box_cons.rs`)

<p align="center">

```rust
#[derive(Debug)] // To allow printing with {:?}
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// Helper function to print the list (recursive)
fn print_list_recursive(list: &List) {
    match list {
        List::Cons(value, next_box) => {
            print!("{} -> ", value);
            print_list_recursive(next_box); // Recurse on the dereferenced Box
        }
        List::Nil => {
            println!("Nil");
        }
    }
}

fn main() {
    // Create a Cons list: 0 -> 1 -> 2 -> 3 -> Nil
    let list = List::Cons(0, Box::new(
                    List::Cons(1, Box::new(
                        List::Cons(2, Box::new(
                            List::Cons(3, Box::new(List::Nil))
                        ))
                    ))
                ));
    println!("List created with nested Cons: {:?}", list);

    // Create another list iteratively for demonstration
    let a = List::Cons(3, Box::new(List::Nil));
    let b = List::Cons(2, Box::new(a));
    let c = List::Cons(1, Box::new(b));
    let head = List::Cons(0, Box::new(c));

    // Iterate through the list using a loop and pattern matching
    println!("Iterating through 'head' list:");
    let mut current_node_ref = &head;
    loop { // Loop until Nil is found
        match current_node_ref {
            List::Cons(value, next_box) => {
                println!("Value: {}", value);
                current_node_ref = next_box; // Move to the next node
            }
            List::Nil => {
                break; // End of list
            }
        }
    }

    println!("\nPrinting 'head' list recursively:");
    print_list_recursive(&head);
}
```

</p>

*(Icon: A simple database cylinder labeled `box_cons.rs`)*

---

## `Box<T>` with Binary Tree Example (Definition and Usage)

```rust
#[derive(Debug)]
enum BinaryTree {
    Empty,
    Node(i32, Box<BinaryTree>, Box<BinaryTree>), // Value, Left child, Right child
}

impl BinaryTree {
    fn new() -> Self { BinaryTree::Empty }

    fn insert(&mut self, value: i32) {
        match *self {
            BinaryTree::Empty => {
                *self = BinaryTree::Node(value, Box::new(BinaryTree::Empty), Box::new(BinaryTree::Empty));
            }
            BinaryTree::Node(ref mut data, ref mut left_child_box, ref mut right_child_box) => {
                if value <= *data { left_child_box.insert(value); }
                else { right_child_box.insert(value); }
            }
        }
    }
}

fn main() { // box_tree.rs
    let mut tree = BinaryTree::new();
    tree.insert(5); tree.insert(3); tree.insert(7);
    tree.insert(1); tree.insert(4); tree.insert(6); tree.insert(8);
    println!("{:?}", tree);
}
```

*(Icon: A simple database cylinder labeled `box_tree.rs`)*
*(Diagram: A binary search tree with 5 at root and children 3, 7, etc., leaves marked E for Empty.)*

---

## `std::rc::Rc<T>` (Reference Counted Pointer)

`Rc<T>`: **single-threaded reference-counting** smart pointer for **multiple owners** of **immutable data**.

*   **Internal Structure:** Pointer to heap data `T` + **strong count** + **weak count**.
*   **Cloning:** `Rc::clone(&rc_instance)` creates new `Rc<T>` to same data, increments **strong count**. Cheap operation.
*   **Dropping:** `Rc<T>` drop decrements strong count. Data `T` dropped when strong count == 0.
*   **Use Cases:** Shared ownership in trees, acyclic graphs.
*   **Limitation:** Single-threaded only (non-atomic). Use `Arc<T>` for multi-threaded.
*   **Method Syntax:** `Rc::method(&rc)` instead of `rc.method()` for methods on `Rc` itself to avoid conflicts with methods on `T`.

---

## `Rc<T>`: Example 1 (`rc.rs`)

<p align="center">

```rust
use std::rc::Rc;

fn main() {
    let rc_example = Rc::new("Example Rc".to_string());
    println!("Initial strong count for rc_example: {}", Rc::strong_count(&rc_example)); // Output: 1
    { // Inner scope 1
        let rc_a = Rc::clone(&rc_example);
        println!("Strong count of rc_a: {}", Rc::strong_count(&rc_a)); // Output: 2
        println!("Strong count of rc_example is also: {}", Rc::strong_count(&rc_example)); // Output: 2
        { // Inner scope 2
            let rc_b = Rc::clone(&rc_a);
            println!("Strong count of rc_b: {}", Rc::strong_count(&rc_b)); // Output: 3
            println!("Strong count of rc_a: {}", Rc::strong_count(&rc_a));   // Output: 3
            println!("Are rc_a and rc_b equal (pointing to same data)? {}", rc_a.eq(&rc_b)); // Output: true
            println!("Length of the value within rc_a: {}", rc_a.len()); // Output: (length of "Example Rc")
        } // rc_b goes out of scope here. strong count becomes 2.
        println!("Strong count of rc_a after rc_b is out of scope: {}", Rc::strong_count(&rc_a)); // Output: 2
    } // rc_a goes out of scope here. strong count becomes 1.
    println!("Strong count of rc_example after rc_a is out of scope: {}", Rc::strong_count(&rc_example)); // Output: 1
} // rc_example goes out of scope here. strong count becomes 0. String "Example Rc" deallocated.
```

</p>

*(Icon: A simple database cylinder labeled `rc.rs`)*

---

## `Rc<T>`: Example with Cons List (`rc3.rs`)

Using `Rc<T>` to share immutable list tails.

<p align="center">

```rust
use std::rc::Rc;

#[derive(Debug)] // To print the list
enum List {
    Cons(i32, Rc<List>), // Each Cons cell holds a value and an Rc to the rest of the list
    Nil,
}

use List::{Cons, Nil}; // Shorthand for variants

fn main() {
    // Create list a: 5 -> 10 -> Nil
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("a initial strong_count = {}", Rc::strong_count(&a)); // Output: 1

    // Create list b: 3 -> a (shares tail with a)
    let b = Rc::new(Cons(3, Rc::clone(&a)));
    println!("a strong_count after b creation = {}", Rc::strong_count(&a)); // Output: 2
    println!("b initial strong_count = {}", Rc::strong_count(&b)); // Output: 1

    // Create list c: 4 -> a (also shares tail with a)
    let c = Rc::new(Cons(4, Rc::clone(&a)));
    println!("a strong_count after c creation = {}", Rc::strong_count(&a)); // Output: 3
    println!("c initial strong_count = {}", Rc::strong_count(&c)); // Output: 1

    println!("\nList a: {:?}", a);
    println!("List b: {:?}", b);
    println!("List c: {:?}", c);
}
```

</p>

*(Icon: A simple database cylinder labeled `rc3.rs`)*
*(Diagram: Shows `a`, `b`, `c` Rc's pointing to Cons cells; `b` and `c`'s tails point to the same Rc as `a`'s tail. Reference counts updated.)*

---

## Mutability of Data Pointed to by `Rc<T>`

Data inside `Rc<T>` is **immutable** by default. `Rc::get_mut(&mut Rc<T>) -> Option<&mut T>` gets `Some(&mut T)` **only if strong count is 1**, allowing mutation. Otherwise, returns `None`. Shared mutability needs `Cell`/`RefCell`.

---

## `Rc::get_mut()` Example (`rc4.rs`)

<p align="center">

```rust
use std::rc::Rc;

fn main() {
    let mut value_rc = Rc::new(5);

    println!("Initial Rc value: {:?}", value_rc); // Output: 5
    println!("Initial strong count: {}", Rc::strong_count(&value_rc)); // Output: 1
    { // Inner scope to demonstrate effect of shared references
        let copy_rc = Rc::clone(&value_rc);
        println!("Strong count after clone: {}", Rc::strong_count(&value_rc)); // Output: 2
        // Attempt to get mutable reference fails because strong count > 1.
        match Rc::get_mut(&mut value_rc) {
            Some(v_mut) => *v_mut += 10,
            None => println!("Cannot get a mutable reference when strong count > 1."), // This will print
        }
    } // 'copy_rc' goes out of scope. Strong count of 'value_rc' drops back to 1.

    // Try to get a mutable reference again. Succeeds.
    match Rc::get_mut(&mut value_rc) {
        Some(v_mut) => {
            *v_mut += 10;
            println!("Value modified successfully via get_mut.");
        }
        None => println!("Still cannot get a mutable reference (should not happen)."),
    }
    println!("Final Rc value: {:?}", value_rc); // Output: 15
    println!("Final strong count: {}", Rc::strong_count(&value_rc)); // Output: 1
}
```

</p>

*(Icon: A simple database cylinder labeled `rc4.rs`)*

---

## `std::rc::Weak<T>` (Weak Pointer)

`Weak<T>`: **non-owning ("weak") reference** to data managed by `Rc<T>`. Used to **break reference cycles** by not contributing to strong count.

*   **Creation:** `Rc::downgrade(&rc_instance)` increments weak count.
*   **Accessing Data:** Must `upgrade()` (`weak_instance.upgrade()`) which returns `Some(Rc<T>)` if data is live (strong count > 0), `None` otherwise. `upgrade` creates a temporary strong reference.
*   **Characteristics:** Generated only from `Rc` (or `Arc`). Not directly dereferencable (`*w` invalid). Must upgrade to access data.

---

## `Weak<T>`: Example 2 (`weak2.rs` - Data Dropped)

<p align="center">

```rust
use std::rc::Rc;

fn main() {
    let five_rc = Rc::new(5); // Strong count = 1
    println!("Initial five_rc: {:?}, strong_count: {}", five_rc, Rc::strong_count(&five_rc));

    // Create Weak pointer. Weak count = 1. Strong count remains 1.
    let weak_five = Rc::downgrade(&five_rc);
    println!("Weak count for five_rc's data: {}", Rc::weak_count(&five_rc));

    // Drop the original Rc<T>. Only strong reference. Strong count becomes 0. Data deallocated.
    drop(five_rc);
    println!("five_rc dropped.");

    // Attempt to upgrade Weak pointer. Data is gone. Returns None.
    let strong_five_option: Option<Rc<i32>> = weak_five.upgrade();
    println!("Attempt to upgrade weak_five after original Rc dropped: {:?}", strong_five_option);
    // Output: Attempt to upgrade weak_five...: None
} // weak_five goes out of scope. Weak count becomes 0. Control block deallocated.
```

</p>

*(Icon: A simple database cylinder labeled `weak2.rs`)*

---

## `std::cell` Module (Interior Mutability)

Rust's compile-time borrow checker enforces (exclusive): 1 owner, or many immutable refs (`&T`), or one mutable ref (`&mut T`).

Sometimes static analysis is too restrictive. `std::cell` containers provide **controlled shared mutability** (interior mutability), allowing data modification through immutable references (`&T`), with borrowing rules checked at **runtime** (panic on violation). Only for **single-threaded** contexts (`!Sync`, `!Send`).

---

## `std::cell::Cell<T>`

*   `Cell<T>`: Allows interior mutability via methods taking `&self`.
*   No extra metadata; very lightweight.
*   Modifies by **replacing entire value** (`set`, `take`, `replace`). Does **not** allow `&T` or `&mut T` references to inner data.
*   `get()` requires `T: Copy`. `take()` requires `T: Default`. `replace()`, `into_inner()` work for any `T: Sized`.

---

## `Cell<T>` Example 1 (`cell.rs`)

<p align="center">

```rust
use std::cell::Cell;

#[derive(Debug)]
struct SomeStruct {
    a: u8,       // Normal mutability rules apply
    b: Cell<u8>, // Interior mutability applies
}

fn main() {
    let my_struct = SomeStruct {
        a: 0,
        b: Cell::new(1),
    };

    // my_struct.a = 100; // ERROR: `my_struct` is immutable
    // OK: even though my_struct is immutable, can modify inner Cell value
    my_struct.b.set(100);

    println!("my_struct: {:?}", my_struct);
    // Output: my_struct: SomeStruct { a: 0, b: Cell { value: 100 } }
}
```

</p>

*(Icon: A simple database cylinder labeled `cell.rs`)*

---

## `Cell<T>` Example 2 (`cell2.rs` - Counter)

<p align="center">

```rust
use std::cell::Cell;

struct Counter {
    count: Cell<u32>,
}

impl Counter {
    fn new() -> Counter { Counter { count: Cell::new(0) } }
    // Takes &self but modifies inner count
    fn increment(&self) { self.count.set(self.count.get() + 1); }
    fn decrement(&self) { self.count.set(self.count.get() - 1); }
    fn read_value(&self) -> u32 { self.count.get() }
}

fn main() {
    let counter = Counter::new();
    counter.increment(); counter.increment();
    println!("Count: {}", counter.read_value()); // Output: Count: 2
    counter.decrement();
    println!("Count: {}", counter.read_value()); // Output: Count: 1
}
```

</p>

*(Icon: A simple database cylinder labeled `cell2.rs`)*

---

## `std::cell::RefCell<T>`

*   `RefCell<T>`: Allows creating **references** (`&T` or `&mut T`) to inner data.
*   Runtime borrow checks: Tracks borrow state (none, immutable, mutable). Panics if borrow rules violated (e.g., multiple mutable or mut+immut).
*   **Methods:**
    *   `borrow(&self) -> Ref<'_, T>`: Returns immutable borrow (`&T` wrapper). Panics if mutable borrow active.
    *   `borrow_mut(&self) -> RefMut<'_, T>`: Returns mutable borrow (`&mut T` wrapper). Panics if *any* borrow active.
    *   `try_borrow(&self) -> Result<Ref<'_, T>, BorrowError>`: Non-panicking immutable borrow.
    *   `try_borrow_mut(&self) -> Result<RefMut<'_, T>, BorrowMutError>`: Non-panicking mutable borrow.
    *   `into_inner(self) -> T`: Consumes `RefCell`, returns inner value. Panics if borrows active.
    *   `get_mut(&mut self) -> &mut T`: Returns direct `&mut T`. Requires `&mut RefCell`, so no runtime check needed.

---

## `RefCell<T>` Example (`refcell.rs`)

<p align="center">

```rust
use std::cell::RefCell;

fn main() {
    let c = RefCell::new(5);
    println!("Initial data: {:?}", c.borrow()); // Output: Initial data: 5
    { // Inner scope
        *c.borrow_mut() += 5; // Mutably borrows, adds 5. Data becomes 10.
        println!("Data after borrow_mut and modification: {:?}", c.borrow()); // Output: 10

        let mut m_refmut = c.borrow_mut(); // Another mutable borrow (panics if previous borrow not dropped)
        if c.try_borrow().is_err() {
            println!("Cannot make another borrow (immutable) while mutable borrow is active."); // This will print
        }
        *m_refmut = 6; // Modify through RefMut
    } // m_refmut drops, releasing mutable borrow.

    if c.try_borrow().is_ok() {
        println!("Can make an immutable borrow now."); // This will print
        let m_ref = c.borrow();
        println!("Data via m_ref: {:?}", *m_ref); // Output: 6
        if c.try_borrow_mut().is_err() {
            println!("Cannot make a mutable borrow while immutable borrow is active."); // This will print
        }
        drop(m_ref); // Explicitly drop m_ref
    }
    let val = c.into_inner(); // Consumes c, returns inner value
    println!("Final value after destroying the RefCell: {:?}", val); // Output: 6
}
```

</p>

*(Icon: A simple database cylinder labeled `refcell.rs`)*

---

## Combining `RefCell<T>` and `Rc<T>` (`rc_refcell.rs`)

`Rc<RefCell<T>>`: data with **multiple owners** (`Rc`) and **interior mutability** (`RefCell`, runtime checked).

<p align="center">

```rust
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    // 'value' is an Rc pointing to a RefCell containing 5.
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Rc::new(Cons(Rc::new(RefCell::new(6)), Rc::clone(&a)));
    let c = Rc::new(Cons(Rc::new(RefCell::new(10)), Rc::clone(&a)));

    println!("a before = {:?}", a);
    println!("b before = {:?}", b);
    println!("c before = {:?}", c);

    // Modify the shared 'value' through its RefCell's mutable borrow.
    *value.borrow_mut() += 10; // value becomes 15

    println!("\na after mutation of shared value = {:?}", a);
    println!("b after mutation of shared value = {:?}", b);
    println!("c after mutation of shared value = {:?}", c);
    // All lists reflect the change to 15 for the shared element.
}
```

</p>

*(Icon: A simple database cylinder labeled `rc_refcell.rs`)*

---

## Example: Doubly Linked Tree (Implementation)

Uses `Rc` for child pointers (strong ownership downwards) and `RefCell<Weak>` for parent pointers (non-owning, for cycles, allows modifying parent reference). `RefCell` allows modifying parent/children lists when node is shared via `Rc`.

<p align="center">

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::fmt; // For custom Debug if needed, or just use #[derive(Debug)]

#[derive(Debug)] // Simplified Debug for presentation
struct Node {
    name: String,
    parent: RefCell<Weak<Node>>, // Weak pointer to parent
    children: RefCell<Vec<Rc<Node>>>, // Strong pointers to children
}

impl Node {
    fn new(name: &str) -> Rc<Self> {
        Rc::new(Node {
            name: name.to_string(),
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(Vec::new()),
        })
    }

    fn add_child(parent_rc: &Rc<Node>, child_rc: &Rc<Node>) {
        *child_rc.parent.borrow_mut() = Rc::downgrade(parent_rc);
        parent_rc.children.borrow_mut().push(Rc::clone(child_rc));
    }

    fn print_tree(node_rc: &Rc<Node>, depth: usize) {
        println!("{}{}", "  ".repeat(depth), node_rc.name);
        for child_rc in node_rc.children.borrow().iter() {
            Node::print_tree(child_rc, depth + 1);
        }
    }

    fn get_root(node_rc: &Rc<Node>) -> Rc<Node> {
        let mut current_node_rc = Rc::clone(node_rc);
        loop {
            match current_node_rc.parent.borrow().upgrade() {
                Some(parent_rc_strong) => current_node_rc = parent_rc_strong,
                None => break,
            }
        }
        current_node_rc
    }
}

fn main() { // albero.rs (tree.rs)
    let root = Node::new("root");
    let child1 = Node::new("child1");
    let child2 = Node::new("child2");
    let subchild = Node::new("subchild");

    Node::add_child(&root, &child1);
    Node::add_child(&root, &child2);
    Node::add_child(&child1, &subchild);

    println!("Tree structure from child1's root:");
    Node::print_tree(&Node::get_root(&child1), 0);
}
```

</p>

*(Icon: A simple database cylinder labeled `albero.rs`)*
*(Text Box shows expected output structure)*

---

## `std::borrow::Cow<'a, B>` (Clone-on-Write Smart Pointer)

`Cow<'a, B>`: implements **clone-on-write**. If data is borrowed (`Cow::Borrowed(&'a B)`) and needs modification, it's cloned to an owned copy (`Cow::Owned(<B as ToOwned>::Owned)`) before modification. If already owned, no cloning. `B: 'a + ToOwned + ?Sized`.

*   `from()`: Creates `Cow` from reference.
*   `to_mut(&mut self) -> &mut <B as ToOwned>::Owned`: Returns mutable reference. Clones if `Borrowed`.
*   `into_owned(self) -> <B as ToOwned>::Owned`: Consumes `Cow`, returns owned value (clones if `Borrowed`).
*   `as_ref(&self) -> &'a B`: Returns immutable reference.

---

## `Cow<'a, str>`: Example 1 (`cow0.rs`)

<p align="center">

```rust
use std::borrow::Cow;

// Converts input to uppercase only if it contains lowercase letters, returning original borrowed otherwise.
fn make_uppercase(input: &str) -> Cow<str> {
    if input.chars().any(|c| c.is_lowercase()) {
        Cow::Owned(input.to_uppercase()) // Clones and returns Owned
    } else {
        Cow::Borrowed(input) // Returns Borrowed, no allocation
    }
}

fn main() {
    let string1 = String::from("Ciao"); // "Hello"
    let string2 = String::from("CIAO"); // "HELLO"
    let result1 = make_uppercase(&string1);
    let result2 = make_uppercase(&string2);
    println!("Result 1: {} (Was Cloned: {})", result1, matches!(result1, Cow::Owned(_)));
    println!("Result 2: {} (Was Cloned: {})", result2, matches!(result2, Cow::Owned(_)));
}
```

</p>

*(Icon: A simple database cylinder labeled `cow0.rs`)*

---

## `Cow<'a, str>`: Example 2 (`cow.rs`)

Avoids cloning if modification isn't needed (length check).

<p align="center">

```rust
use std::borrow::Cow;

// Modifies content to lowercase if short, otherwise returns as is.
fn modify_if_condition(content: Cow<str>) -> Cow<str> {
    if content.len() < 10 {
        Cow::Owned(content.into_owned().to_lowercase()) // Ensure owned, then lowercase
    } else {
        content // Return Cow as is
    }
}

fn main() {
    let long_string = String::from("This is a long string");
    let short_string = String::from("SHORT");
    let modified_long = modify_if_condition(Cow::from(&long_string));
    println!("Modified long: {}", modified_long); // Output: This is a long string (Borrowed)
    let modified_short = modify_if_condition(Cow::from(&short_string));
    println!("Modified short: {}", modified_short); // Output: short (Owned after clone/lowercase)
}
```

</p>

*(Icon: A simple database cylinder labeled `cow.rs`)*

---

## `Cow<'a, str>`: Example with `to_mut()` (`cow3.rs`)

<p align="center">

```rust
use std::borrow::Cow;

fn main() {
    let mut borrowed_str_data: Cow<str> = Cow::Borrowed("Salve Mondo!"); // "Hello World!"

    // Call to_mut() on a borrowed Cow<str>.
    // If Borrowed, converts to Owned (cloning), allowing mutable access.
    let mutable_string_slice = borrowed_str_data.to_mut(); // mutable_string_slice is &mut String

    mutable_string_slice.make_ascii_uppercase(); // Mutate the data.

    println!("Mutated data: {}", mutable_string_slice); // Output: Mutated data: SALVE MONDO!
    println!("Original Cow data (now owned and uppercase): {}", borrowed_str_data); // Output: SALVE MONDO!
}
```

</p>

*(Icon: A simple database cylinder labeled `cow3.rs`)*

---

## Smart Pointers and Methods (`self` argument types)

Methods can take `self: Box<Self>`, `self: Rc<Self>`, or `self: Arc<Self>`, consuming the smart pointer (and owned data). Only callable on the smart pointer type.

**Example (`self1.rs`):**
```rust
struct Task { name: String, }
impl Task {
    // Takes ownership of the Box<Self>
    fn complete(self: Box<Self>) {
        println!("Task '{}' completed and dropped!", self.name);
    }
}
fn main() {
    let task_box = Box::new(Task { name: String::from("Study Smart Pointers") });
    task_box.complete(); // Moves ownership of task_box
    // task_box cannot be used here.
}
```

*(Icon: A simple database cylinder labeled `self1.rs`)*

---

## `Rc<Self>` in Methods (`self2.rs`)

<p align="center">

```rust
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Rc<Node>>,
}

impl Node {
    // Takes ownership of the Rc<Self> and iterates through the chain
    fn print_chain(self: Rc<Self>) { // self is Rc<Node>
        let mut current_option_rc = Some(self);
        while let Some(current_rc_node) = current_option_rc {
            println!("{}", current_rc_node.value);
            // Clone the next Rc (if it exists) to move to the next node
            current_option_rc = current_rc_node.next.as_ref().map(|rc_next_node| Rc::clone(rc_next_node));
            // The loop implicitly drops the Rc for the previous node.
        }
    }
}

fn main() {
    let third = Rc::new(Node { value: 3, next: None });
    let second = Rc::new(Node { value: 2, next: Some(Rc::clone(&third)) });
    let first = Rc::new(Node { value: 1, next: Some(Rc::clone(&second)) });
    first.print_chain(); // Prints 1, 2, 3, consumes the 'first' Rc.
}
```

</p>

*(Icon: A simple database cylinder labeled `self2.rs`)*