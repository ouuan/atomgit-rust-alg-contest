use rand::{thread_rng, Rng};
use std::collections::HashSet;

/// 概率去重函数
pub fn probability_count_distinct(input_str: &str) -> usize {
    let parts: Vec<&str> = input_str.split(",").collect();
    let thresh = parts.len();
    let mut p = 1.0;
    let medium = 0.5;
    let mut seen: HashSet<&str> = HashSet::new();

    let mut rng = thread_rng();
    for value in parts.iter() {
        seen.remove(value);
        if rng.gen::<f64>() < p {
            seen.insert(value);
        }

        // Objects now need to win an extra coin flip to be included
        // in the set. Every element in `seen` already won n-1 coin
        // flips, so they now have to win one more.
        //
        // Reset the set with elements that win an extra coin flip
        if seen.len() == thresh {
            seen = seen
                .drain()
                .filter(|&_e| rng.gen::<f64>() < medium)
                .collect();
            p *= medium;
        }
    }

    (seen.len() as f64 / p) as usize
}
