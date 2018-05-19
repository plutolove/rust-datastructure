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
### Example of Vector
```
macro_rules! vec_of_D {
    ($($elem: expr), *) => {
        {
            let mut v = Vector::new();
            $(
                v.push(D{x: $elem});
            )*
            v
        }
    };
}
fn main() {
    let mut y = vector![1, 3, 4, 56, 76];
    y[0] = 34;
    assert_eq!(y[0], 34);
    y.push(456);
    y.pop();
}
```