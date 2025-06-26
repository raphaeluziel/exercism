// https://rust-unofficial.github.io/too-many-lists

type Link = Option<Box<Node>>;

pub struct SimpleLinkedList {
    head: Link
}

struct Node {
    data: u8,
    next: Link,
}

impl SimpleLinkedList {
    pub fn new() -> Self {
        Self { head: None }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!()
    }

    pub fn push(&mut self, element: u8) {
        let new_node = Box::new(Node { data: element, next: self.head.take() });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<u8> {
        self.head.take().map(|node| { self.head = node.next; node.data })
    }

    pub fn peek(&self) -> Option<&u8> {
        todo!()
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList {
        todo!()
    }
}

impl FromIterator<u8> for SimpleLinkedList {
    fn from_iter<I: IntoIterator<Item = u8>>(_iter: I) -> Self {
        todo!()
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
//
// The reason this exercise's API includes an explicit conversion to Vec instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.
//
// Please note that the "front" of the linked list should correspond to the "back"
// of the vector as far as the tests are concerned.

impl From<SimpleLinkedList> for Vec<u8> {
    fn from(mut _linked_list: SimpleLinkedList) -> Vec<u8> {
        todo!()
    }
}
