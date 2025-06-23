#[derive(Debug)]
enum Link<i32> {
    Cons(i32, Box<Link<i32>>),
    Nil
}

#[derive(Debug)]
struct SLL<i32> {
    last_link: Link<i32>
}

impl SLL<i32> {
    pub fn new() -> Self {
        SLL { last_link: Link::Nil }
    }
    pub fn is_empty(&self) -> bool {
        self.last_link
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn push(&mut self, _element: i32) {
        let new_link = Link::Nil;
        self.last_link = Link::Cons(_element, Box::new(new_link));
    }
}


fn main() {
    let mut sll: SLL<i32> = SLL::new();
    sll.push(244);
    println!("\n{:?}\n", sll);

    sll.push(683);

    println!("\n{:?}\n", sll);
}