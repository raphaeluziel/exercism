use std::collections::BTreeMap;
pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    BTreeMap::from_iter(
        h.iter()
         .flat_map(|x| x.1.iter()
                          .map(move |&y| (y.to_ascii_lowercase(), *x.0))))
}