use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut hashset: HashSet<i32> = HashSet::new();

    for num in nums {
        if !hashset.insert(num) {
            return true;
        }
    }

    false
}