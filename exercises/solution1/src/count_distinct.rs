use std::collections::HashSet;

pub fn new_count_distinct(input_str: &str) -> usize {
    input_str.split(',').collect::<HashSet<_>>().len()
}
