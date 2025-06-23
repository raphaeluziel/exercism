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
        if !self.is_empty() {
            self.data.last_mut().unwrap().1 = std::ptr::from_ref(&_element);
        }
        self.data.push((_element, std::ptr::null()));
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } 
        else {
            let popped = self.data.pop().unwrap().0;

            if !self.is_empty() {
                self.data.last_mut().unwrap().1 = std::ptr::null();
            }

            Some(popped)
        }
    }

    pub fn peek(&self) -> Option<&T> {
        if self.data.is_empty() {
            None
        }
        else {
            Some(&self.data.last().unwrap().0)
        }
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut rsll: SimpleLinkedList<T> = SimpleLinkedList::new();

        let mut reversed_list = self.data;
        reversed_list.reverse();

        for reversed_element in reversed_list {
            rsll.push(reversed_element.0);
        }

        rsll
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut sll: SimpleLinkedList<T> = SimpleLinkedList::new();

        for i in _iter {
            sll.push(i);
        }

        sll
    }
}


impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        _linked_list
            .data
            .into_iter()
            .map(|(element, _pointer)| element)
            .collect()
    }
}
