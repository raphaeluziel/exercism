pub struct SimpleLinkedList<T> {
    data: Vec<T>,
    index: usize
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { data: Vec::new(), index: 0 }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn push(&mut self, _element: T) {
        self.data.push(_element);
        self.index += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.data.is_empty() { None }
        else {
            self.index -= 1;
            self.data.pop()
        }
    }

    pub fn peek(&self) -> Option<&T> {
        if self.data.is_empty() { None }
        else { Some(&self.data[self.index - 1]) }
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut reversed_list = self.data;
        reversed_list.reverse();
        SimpleLinkedList { data: reversed_list, index: self.index }
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        todo!()
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.
//
// Please note that the "front" of the linked list should correspond to the "back"
// of the vector as far as the tests are concerned.

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        _linked_list.data
    }
}
