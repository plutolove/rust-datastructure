## Rust implementation of data structure
[![Build Status](https://travis-ci.org/plutolove/rust-datastructure.svg?branch=master)](https://travis-ci.org/plutolove/rust-datastructure)
#### LinkedList
```rust
fn main() {
    let mut list: LinkedList<u32> = LinkedList::new();
    list.push_front(0);
    list.push_back(1);
    list.push_back(2);
    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&0));
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), None);
    list.clear();
}
```