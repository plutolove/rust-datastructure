#![feature(box_into_raw_non_null)]
#![feature(dropck_eyepatch)]
pub mod collections;



#[cfg(test)]
mod tests {
    pub use collections::linked_list::LinkedList;
    #[test]
    fn test_push_pop_front() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        list.push_front(4);
        assert_eq!(list.len(), 4);
        list.pop_front();
        assert_eq!(list.len(), 3);
    }
    #[test]
    fn test_push_pop_back() {
        let mut list = LinkedList::new();
        list.push_back(2);
        list.push_back(54);
        list.push_back(3556);
        list.push_back(45656);
        assert_eq!(list.len(), 4);
        list.pop_back();
        assert_eq!(list.len(), 3);
    }
    #[test]
    fn test_iter() {
        let mut list: LinkedList<u32> = LinkedList::new();

        list.push_back(0);
        list.push_back(1);
        list.push_back(2);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), None);
    }
}
