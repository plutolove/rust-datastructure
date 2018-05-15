#![feature(box_into_raw_non_null)]
#![feature(dropck_eyepatch)]
mod collections;
use collections::linked_list::LinkedList;

fn main() {
    let mut list = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    list.push_back(4);

    for item in list.iter() {
        println!("{}", item);
    }
}