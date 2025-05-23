use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    BTreeMap::from_iter(
        h.iter().flat_map(|(&n, vc)| 
            vc.iter().map(move |&ch| (ch.to_ascii_lowercase(), n))))
}