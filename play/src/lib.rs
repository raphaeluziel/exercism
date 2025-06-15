pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let upper_bound = 13u64;

    let mut marked: Vec<bool> = vec![true; upper_bound as usize];
    //marked[0] = true;   // 2 is prime, and will be first number checked

    let mut p = 2u64;
    let mut n = p * p;

    while p <= upper_bound {
        println!("P  = {p}, n  = {n}");
        while n <= upper_bound {
            marked[n as usize] = false;
            n += p;
        }
        p += 1;
        n = p * p;
        println!("PP = {p}, nn = {n}");
    }

    let vvv: Vec<u64> = marked.iter().enumerate().filter(|&(i, v)| i > 1 && *v).map(|(i, _)| i as u64).collect();

    println!("Marked B = {:#?}", marked);
    println!("VVV = {:#?}", vvv);

    //todo!("Construct a vector of all primes up to {upper_bound}");
    vvv
}