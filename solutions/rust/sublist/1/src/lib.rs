use crate::Comparison::Unequal;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    // obviously the same
    if first_list == second_list {
        return Comparison::Equal;
    }
    if first_list.is_empty() {
        return Comparison::Sublist;
    } else if second_list.is_empty() {
        return Comparison::Superlist;
    }
    if first_list.len() < second_list.len() {
        if second_list
            .windows(first_list.len())
            .any(|w| w == first_list)
        {
            return Comparison::Sublist;
        }
    }
    if first_list.len() > second_list.len() {
        if first_list
            .windows(second_list.len())
            .any(|w| w == second_list)
        {
            return Comparison::Superlist;
        }
    }
    // bogus return
    if first_list.len() != second_list.len() {
        return Comparison::Unequal;
    }

    // if all fails
    Comparison::Unequal
}
// pub fn sublist(first_list: &[1 2 3], second_list: &[1 3]) -> Comparison {
//     if first_list.len() > second_list.len() {
//         if second_list.iter().all(|x| first_list.contains(x)) {
//             return Comparison::Superlist;
//         }
//     }
//     // bogus return
//     if first_list.len() != second_list.len() {
//         return Comparison::Unequal;
//     }

//     // if all fails
//     Comparison::Unequal
// }
