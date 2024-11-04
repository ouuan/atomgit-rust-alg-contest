const CHOICES: [u32; 8] = [1, 2, 5, 10, 20, 30, 50, 100];

/// dynamic programming
pub fn dp_rec_mc(amount: u32) -> u32 {
    let mut dp = vec![u32::MAX; (amount + 1) as usize];
    dp[0] = 0;
    for i in 1..=amount {
        for x in CHOICES.iter().take_while(|x| **x <= i) {
            dp[i as usize] = dp[i as usize].min(dp[(i - x) as usize] + 1);
        }
    }
    dp[amount as usize]
}

#[allow(dead_code)]
/// greedily always choose the maximum possible value
pub fn greedy_rec_mc(amount: u32) -> u32 {
    let mut remain = amount;
    let mut count = 0;
    for x in CHOICES.iter().rev() {
        count += remain / x;
        remain %= x;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greedy_dp_matches() {
        (0..200).for_each(|i| {
            assert_eq!(dp_rec_mc(i), greedy_rec_mc(i), "does not match for i = {i}");
        });
    }
}
