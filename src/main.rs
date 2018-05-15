#![feature(box_into_raw_non_null)]
mod collections;
use collections::linked_list::LinkedList;

fn main() {
    let mut list = LinkedList::new();
    list.push_back(1);
    list.push_back(1);
    list.push_back(1);
    list.push_back(1);
}