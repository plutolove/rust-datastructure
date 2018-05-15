## Rust implementation of data structure
[![Build Status](https://travis-ci.org/plutolove/rust-datastructure.svg?branch=master)](https://travis-ci.org/plutolove/rust-datastructure)
#### Example of LinkedList
```rust
fn main() {
    let mut ls = LinkedList::new();
    ls.push_back(46);
    ls.push_back(45);
    ls.push_back(43);
    //mut iterator
    for item in ls.iter_mut() {
        *item += 10;
    }
    //immt iterator
    let mut it = ls.iter();
    assert_eq!(it.next(), Some(&56));
    assert_eq!(it.next(), Some(&55));
    assert_eq!(it.next(), Some(&53));
    assert_eq!(it.next(), None);
}
```