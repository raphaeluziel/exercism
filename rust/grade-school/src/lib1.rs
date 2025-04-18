use std::collections::BTreeMap;

#[derive(Debug)]
pub struct School<'a> {
    roster: BTreeMap<&'a str, u32>
}

impl<'a> School<'a> {
    pub fn new() -> School<'a> {
        School { roster: BTreeMap::new() }
    }

    pub fn add(&mut self, grade: u32, student: &'a str) {
        if !self.roster.contains_key(student) {
            self.roster.insert(student, grade);
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut grades:Vec<u32> = self.roster.values().cloned().collect();
        grades.dedup();
        grades
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        self.roster.iter()
                    .filter(|&(ref _s, &grd)| grd == grade)
                    .map(|(s, _g)| s.to_string())
                    .collect()
    }
}