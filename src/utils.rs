use std::fmt::Debug;

#[allow(dead_code)]
pub fn assert_sort<K: Ord + Debug>(vec: Vec<K>) {
    for i in 1..vec.len() {
        match vec[i] < vec[i-1] {
            true => panic!("Elements at indices {} and {} are not ordered correctly ({:?} < {:?})", i, i-1, vec[i], vec[i-1]),
            _ => ()
        }
    }
}