#[derive(Debug)]
enum Link<i32> {
    Cons(i32, Box<Link<i32>>),
    Nil
}
#[derive(Debug)]
struct SLL<i32> {
    data: Link<i32>
}

impl SLL<i32> {
    pub fn new() -> Self {
        Self { data: Link::Nil }
    }
    pub fn push(&mut self, _element: i32) {
        let mut ssllll: SLL<i32> = SLL { data: Link::Cons(_element, Box::new(Link::Nil)) };
        println!("ssllll = {:?}", ssllll);
    }
}


fn main() {
    //let SLL = Data(1, Box::new(Data(2, Box::new(Data(3, Box::new(Nil))))));

    let mut sll: SLL<i32> = SLL::new();
    sll.push(244);

    println!("\n{:?}\n", sll);
}