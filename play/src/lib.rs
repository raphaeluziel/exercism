pub struct SimpleLinkedList<T> {
    data: Vec<(T, *const T)>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn push(&mut self, _element: T) {
        // Set the last pointer, currently null to point to the next element
        // that will be pushed into the SimpleLinkedList
        if !self.is_empty() {
            self.data.last_mut().unwrap().1 = std::ptr::from_ref(&_element);
        }

        // Push the element into the SimpleLinkedList, setting the pointer to
        // null since this will be the last link
        self.data.push((_element, std::ptr::null()));
    }

    pub fn pop(&mut self) -> Option<T> {
        // If the SimpleLinkedList is empty, there's nothing to pop out
        if self.is_empty() {
            None
        } 
        else {
            // Pop out the last element, and save it to return
            let popped = self.data.pop().unwrap().0;

            // If after popping out the last element, the SimpleLinkedList
            // is still not empty, then set the now last element's pointer
            // to null
            if !self.is_empty() {
                self.data.last_mut().unwrap().1 = std::ptr::null();
            }

            // Return the previously popped element
            Some(popped)
        }
    }

    pub fn peek(&self) -> Option<&T> {
        // If the SimpleLinkedList is empty, there's nothing to peek
        if self.data.is_empty() {
            None
        }
        // Otherwise, return the last element, so it can be peeked
        else {
            Some(&self.data.last().unwrap().0)
        }
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        // let mut reversed_list = self.data;
        // reversed_list.reverse();
        // SimpleLinkedList { data: reversed_list, index: self.index }
        todo!()
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        _iter.into_iter()
        // let mut sll = SimpleLinkedList::new();
        // for i in _iter {
        //     sll.data.push((i, ));
        // }
        // sll
    }
}

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        _linked_list.data.into_iter().map(|(element, _pointer)| element).collect()
    }
}
