use std::iter::FromIterator;

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    item: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(item: T, next: Option<Box<Node<T>>>) -> Self {
        Node { item, next }
    }
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    pub fn len(&self) -> usize {
        let mut len = 0;
        //let mut current: &Option<Box<Node<T>>> = &(self.head);
        let mut current: Option<&Box<Node<T>>> = self.head.as_ref();

        while let Some(node_ptr) = current {
            len += 1;
            current = node_ptr.next.as_ref();
        }

        len
    }

    // We push onto head
    pub fn push(&mut self, element: T) {
        let new_head = Box::new(Node::new(element, self.head.take()));
        self.head = Some(new_head);
    }

    pub fn pop(&mut self) -> Option<T> {
        let head = self.head.take(); // need ownership (temporarily)
        if let Some(head_box) = head {
            self.head = head_box.next;
            Some(head_box.item)
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match self.head {
            Some(ref box_node) => Some(&box_node.item),
            None => None,
        }
        //if let Some(head_box) = self.head.as_ref() {
        //    Some(&head_box.item)
        //} else {
        //    None
        //}
    }

    // Don't need as_ref and clone since we are consuming the old SimpleLinkedList
    // Added mut to method param
    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let mut ll = SimpleLinkedList::new();

        while let Some(item) = self.pop() {
            ll.push(item);
        }
        ll
        //let mut ll = SimpleLinkedList::new();
        //let mut current = self.head;

        //while let Some(node_box) = current {
        //    ll.push(node_box.item);
        //    current = node_box.next;
        //}
        //ll
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut ll = SimpleLinkedList::new();
        let mut iter = iter.into_iter();
        while let Some(item) = iter.next() {
            ll.push(item)
        }
        ll
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        //let mut vec = Vec::with_capacity(self.length);
        let mut vec = Vec::new();
        let mut ll = self.rev();
        while let Some(item) = ll.pop() {
            vec.push(item);
        }
        vec
    }
}
