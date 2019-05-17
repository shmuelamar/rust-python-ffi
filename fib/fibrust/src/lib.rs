pub struct Fib {
    a: u64,
    b: u64,
}

impl Fib {
    pub fn new() -> Fib {
        Fib { a: 0, b: 1 }
    }
}

impl Iterator for Fib {
    type Item = u64;

    /// returns the next fibonacci number in the series
    fn next(&mut self) -> Option<Self::Item> {
        let a = self.a;
        self.a = self.b;
        self.b = a + self.b;
        Some(a)
    }
}

/// solves euler-project question #2
pub fn sum_fib_even(max_num: u64) -> u64 {
    let (mut a, mut b) = (0, 1);
    let mut tmp;
    let mut sum = 0;

    while a < max_num {
        if a % 2 == 0 {
            sum += a;
        }
        tmp = a;
        a = b;
        b += tmp;
    }
    sum
}

/// solves euler-project question #2
pub fn sum_fib_even_functional(max_num: u64) -> u64 {
    let res = Fib::new()
        .take_while(|x| x < &max_num)
        .filter(|x| x % 2 == 0)
        .sum();
    res
}

pub fn sum_fib_10k(max_num: u64) -> u64 {
    let mut res = 0;
    for _ in 0..10000 {
        res = sum_fib_even(max_num);
    }
    res
}

pub fn sum_fib_10k_fun(max_num: u64) -> u64 {
    let mut res = 0;
    for _ in 0..10000 {
        res = sum_fib_even_functional(max_num);
    }
    res
}

#[cfg(test)]
mod test {
    use super::Fib;

    #[test]
    fn test_fib_enumerate() {
        let fib_results = vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34];

        for (i, x) in Fib::new().take(10).enumerate() {
            assert_eq!(fib_results[i], x);
        }
    }
}