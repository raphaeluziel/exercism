// Third iteration using format!

pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb = String::new();

    for i in 0..list.len() {
        proverb = if i + 1 < list.len() {
            format!("{}For want of a {} the {} was lost.\n", proverb, list[i], list[i+1])
        }
        else {
            format!("{}And all for the want of a {}.", proverb, list[0])
        }
    }
    
    proverb
}