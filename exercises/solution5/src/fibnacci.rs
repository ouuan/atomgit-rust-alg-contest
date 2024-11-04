struct FibonacciGen {
    a: u32,
    b: u32,
}

impl FibonacciGen {
    pub fn new() -> Self {
        Self { a: 0, b: 1 }
    }
}

impl Iterator for FibonacciGen {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let a = self.a;
        self.a = self.b;
        self.b += a;
        Some(a)
    }
}

pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    FibonacciGen::new()
        .take_while(|&x| x <= threshold)
        .filter(|&x| x % 2 == 1)
        .sum()
}
