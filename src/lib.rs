#![feature(box_into_raw_non_null)]
#![feature(dropck_eyepatch)]
#![feature(ptr_internals)]
#![feature(allocator_api)]
#![feature(unique)]
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

        let mut ls = LinkedList::new();
        ls.push_back(46);
        ls.push_back(45);
        ls.push_back(43);

        for item in ls.iter_mut() {
            *item += 10;
        }
        let mut it = ls.iter();
        assert_eq!(it.next(), Some(&56));
        assert_eq!(it.next(), Some(&55));
        assert_eq!(it.next(), Some(&53));
        assert_eq!(it.next(), None);
    }
    #[test]
    fn test_isempty() {
        let mut list: LinkedList<u32> = LinkedList::new();
        list.push_back(0);
        list.push_back(1);
        list.push_back(2);
        assert_eq!(list.is_empty(), false);
        list.pop_back();
        list.pop_back();
        list.pop_back();
        assert_eq!(list.is_empty(), true);
    }

    pub use collections::vector::Vector;

    macro_rules! vector {
    ($($elem: expr), *) => {
        {
            let mut v = Vector::new();
            $(
                v.push($elem);
            )*
            v
        }
    };
}

    #[test]
    fn test_push() {
        let mut x = vector![1, 2, 3, 4, 5];
        assert_eq!(x.len(), 5);
    }

    #[test]
    fn test_pop() {
        let mut x = vector![1, 2, 3, 4, 5];
        assert_eq!(x.len(), 5);
        let res = x.pop();
        assert_eq!(x.len(), 4);
        assert_eq!(res, Some(5));
        let res = x.pop();
        assert_eq!(x.len(), 3);
        assert_eq!(res, Some(4));
    }
    #[derive(Debug, PartialEq)]
    struct D<T> {
        x: T,
    }

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

    #[test]
    fn test_index() {
        let x = vector![1, 2, 3, 4, 5];
        assert_eq!(x[0], 1);
        assert_eq!(x[1], 2);
        assert_eq!(x[2], 3);
        assert_eq!(x[3], 4);

        let y = vec_of_D![1, 2, 3, 4, 5];
        let z = D { x: 1 };
        assert_eq!(y[0], z)
    }

    #[test]
    fn test_indexmut() {
        let mut y = vector![1, 3, 4, 56, 76];
        y[0] = 34;
        assert_eq!(y[0], 34);
    }

    pub use collections::bitmap::BitMap;
    fn init_bitmap(size: usize) -> BitMap {
        BitMap::new(size)
    }

    #[test]
    fn test_bitmap() {
        let mut b = init_bitmap(10 as usize);
        b.set(0);
        assert_eq!(b.is_set(0), true);
        b.set(1);
        assert_eq!(b.is_set(1), true);

        for i in 0..4 {
            b.set(i as usize);
        }
        for i in 0..4 {
            assert_eq!(b.is_set(i as usize), true);
        }

        for i in 4..10 {
            assert_eq!(b.is_set(i), false);
        }

        for i in 0..4 {
            b.unset(i);
            assert_eq!(b.is_set(i), false);
        }
    }
}
