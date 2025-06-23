use std::ops::Deref;

#[derive(Debug)]
struct SLL<T>(Link<T>);

#[derive(Debug)]
enum Link<T> {
    Cons(T, Box<Link<i32>>),
    Nil
}

impl<T> SLL<T> {
    pub fn new() -> Self {
        Self(Link::Nil)
    }
    pub fn push(&mut self, _element: T) {
        
    }
}

impl<T> Deref for SLL<T> {
    type Target = Link<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


fn main() {
    let mut sll: SLL<i32> = SLL::new();
    sll.push(244);
    //println!("\n{:?}\n", sll);

    sll.push(683);

    sll.push(8);

    //println!("\n{:?}\n", sll);
}