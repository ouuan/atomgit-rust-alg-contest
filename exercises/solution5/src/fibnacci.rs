/// Generate Fibonacci numbers that fit into u32. First items are 1, 1, 2, 3, 5...
struct FibonacciGen {
    a: u32,
    b: u32,
}

impl FibonacciGen {
    pub fn new() -> Self {
        Self { a: 1, b: 0 }
    }
}

impl Iterator for FibonacciGen {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.a.checked_add(self.b)?;
        self.a = self.b;
        self.b = c;
        Some(c)
    }
}

pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    FibonacciGen::new()
        .take_while(|&x| x <= threshold)
        .filter(|&x| x % 2 == 1)
        .sum()
}
