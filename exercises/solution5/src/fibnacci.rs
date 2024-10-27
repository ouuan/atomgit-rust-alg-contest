pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    let mut i = 2; // 已经有2个元素
    let mut fib = vec![0, 1];
    let mut sum = 1;

    loop {
        let c = fib[i - 1] + fib[i - 2];
        if c >= threshold {
            break;
        }
        fib.push(c);
        if c % 2 == 1 {
            sum += c;
        }
        i += 1;
    }

    sum
}
