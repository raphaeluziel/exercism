#[derive(Debug)]
enum Link<i32> {
    Cons(i32, Box<Link<i32>>),
    Nil
}

#[derive(Debug)]
struct SLL<i32> {
    first_link: Link<i32>
}

impl SLL<i32> {
    pub fn new() -> Self {
        SLL { first_link: Link::Nil }
    }
    pub fn push(&mut self, _element: i32) {
        
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