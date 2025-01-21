#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let a = [1, 2, 3, 9 ];
    let b = [990, 880, 1, 2, 3, 9, 110, 220, 330, 440];

    let sm: &[T] = if _first_list.len() <= _second_list.len() { &_first_list } else { &_second_list };
    let lg: &[T] = if _first_list.len() > _second_list.len() { &_first_list } else { &_second_list };

    if sm == lg {
        return Comparison::Equal;
    }

    let smi = sm.len();
    let lgi = lg.len();

    let cp = if _first_list.len() > _second_list.len() { Comparison::Superlist } 
    else if _first_list.len() < _second_list.len() { Comparison::Sublist }
    else { Comparison::Unequal };

    for i in 0..=(lgi - smi) {
        let lg_slice: &[T] = &lg[i..(i + smi)];
        if sm == lg_slice {
            return cp;
        }
    }

    Comparison::Unequal
}