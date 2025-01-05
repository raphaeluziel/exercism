use unicode_reverse::reverse_grapheme_clusters_in_place;

pub fn reverse(input: &str) -> String {
    // todo!("Write a function to reverse {input}");

    // let s = input.chars().rev();
    // let mut r = String::new();
    // for c in s {
    //     r.push(c);
    // }
    // r

    let mut s = input.to_string();
    reverse_grapheme_clusters_in_place(&mut s);
    s
}
