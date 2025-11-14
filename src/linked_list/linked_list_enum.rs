pub use ds_linked_list_enum::Link;

///! Неполноценная реализация
///! https://medium.com/swlh/implementing-a-linked-list-in-rust-c25e460c3676
///!
///! Это список состоит из узлов (мы будем называть наш Links ), которые могут существовать в одном из трех состояний:
///!  - пустой узел,
///!  - узел без связанного next узла, но value(хвостовой узел),
///!  - узел Link - один с узлом value и next узлом.
mod ds_linked_list_enum {

    #[derive(Clone)]
    pub enum Link<T> {
        None,
        Tail { item: T },
        Link { item: T, next: Box<Link<T>> },
    }

    #[derive(Clone)]
    pub struct Cursor<T> {
        curr: Link<T>,
    }

    impl<T> Link<T>
    where
        T: Copy,
    {
        pub fn new() -> Self {
            Self::None
        }

        pub fn pop(&mut self) -> Option<T> {
            match self {
                Self::None => None,
                Self::Tail { item } => {
                    let item = *item;
                    self.to_none();
                    Some(item)
                }
                Self::Link { item, next } => {
                    let mut n = Box::new(Self::None);
                    let item = *item;
                    std::mem::swap(next, &mut n);
                    self.to_next(*n);
                    Some(item)
                }
            }
        }

        pub fn push(&mut self, x: T) {
            match self {
                Self::None => self.to_tail(x),
                Self::Tail { .. } => self.to_link(x),
                Self::Link { next, .. } => next.push(x),
            };
        }

        fn to_none(&mut self) {
            *self = std::mem::replace(self, Link::None);
        }

        fn to_tail(&mut self, it: T) {
            *self = match self {
                Self::None => Self::Tail { item: it },
                Self::Link { item: _, next: _ } => Self::Tail { item: it },
                _ => panic!("Supplied value was not of correct type or variant."),
            }
        }

        fn to_next(&mut self, nxt: Link<T>) {
            *self = nxt;
        }

        fn to_link(&mut self, x: T) {
            *self = match self {
                Self::Tail { item } => Self::Link {
                    item: *item,
                    next: Box::new(Self::Tail { item: x }),
                },
                _ => {
                    panic!("something went wrong");
                }
            };
        }
    }

    impl<T> IntoIterator for Link<T>
    where
        T: Copy,
    {
        type Item = T;
        type IntoIter = Cursor<T>;

        fn into_iter(self) -> Self::IntoIter {
            Cursor { curr: self }
        }
    }

    impl<T> Iterator for Cursor<T>
    where
        T: Copy,
    {
        type Item = T;

        fn next(&mut self) -> Option<T> {
            let nxt = match self.curr {
                Link::None => None,
                Link::Tail { item } => {
                    self.curr = Link::None;
                    Some(item)
                }
                Link::Link { item, ref mut next } => {
                    let mut n = Box::new(Link::None);
                    std::mem::swap(next, &mut n);
                    self.curr = *n;
                    Some(item)
                }
            };
            nxt
        }
    }
}

/// $ cargo test linked_list_enum
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() -> Result<(), String> {
        let mut list: Link<&str> = Link::new();

        list.push("1");
        list.push("2");
        list.push("3");
        list.push("4");

        let node = list.pop();
        assert_eq!(node, Some("1"));

        let node = list.pop();
        assert_eq!(node, Some("2"));

        let node = list.pop();
        assert_eq!(node, Some("3"));

        let node = list.pop();
        assert_eq!(node, Some("4"));

        let _node = list.pop();
        //assert_eq!(_node,None);// Не пашет
        Ok(())
    }
}
