use std::mem;

pub struct  List{
    head: Link,
}

impl List {
    pub fn new() -> Self {
        List {
            head: Link::Empty
        }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Node{
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        };
        self.head = Link::More(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty)  {
            Link::Empty => {
                None
            }
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

pub trait Drop {
    fn drop(&mut self);
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut _boxed_node) = cur_link {
            cur_link = mem::replace(&mut _boxed_node.next, Link::Empty);

        }
    }

}

impl Drop for Link {
    fn drop(&mut self) {
        match self {
            Link::Empty => {
                println!("Drop Empty! Finnal Relefe");
            }
            Link::More(_node) => {
                println!("Drop link: with node with value: {}", _node.elem);
            }
        }
    }
}

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Debug)]
struct Node {
    elem: i32,
    next: Link,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new_empty_list() {
        let list: List = List::new();
        if let Link::Empty = list.head{

        } else {
            panic!("Expected empty list.");
        }
    }

    #[test]
    fn push_add_element_to_list() {
        let mut list = List::new();
        list.push(10);

        match &list.head {
            Link::Empty => panic!("Expected non-empty list after push"),
            Link::More(node) => {
                assert_eq!(node.elem, 10);
                if let Link::Empty = node.next {

                } else {
                    panic!("Expected that new added element is last one!");
                }
            }
        }
    }

    #[test]
    fn pop_elemenet_from_list() {
        let mut list = List::new();
        list.push(10);
        let val: Option<i32> = list.pop();
        match val {
            Option::None => panic!("We are expeting somtinh from non-empty list"),
            Option::Some(num_val) => assert_eq!(num_val, 10),
        }

    }

    #[test]
    fn multiple_push() {
        let mut list = List::new();

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));

        list.push(4);

        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));

        assert_eq!(list.pop(), None);

    }

    #[test]
    fn drop_non_empty_list() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);

        list.drop();

    }
}
