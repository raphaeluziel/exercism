// First try.  I think I can do better, so look for my next iteration to come!

pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb = String::new();
    let starting = "For want of a ";
    let the = " the ";
    let ending = " was lost.\n";

    for i in 0..list.len() {
        if i + 1 < list.len() {
            proverb.push_str(starting);
            proverb.push_str(list[i]);
            proverb.push_str(the);
            proverb.push_str(list[i + 1]);
            proverb.push_str(ending);
        }
        else {
            proverb.push_str("And all for the want of a ");
            proverb.push_str(list[0]);
            proverb.push_str(".");
        }
    }
    proverb
}