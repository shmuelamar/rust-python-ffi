#![feature(test)]
extern crate fibrust;
extern crate test;

use test::Bencher;
use fibrust::Fib;


#[bench]
fn sum_fib_even_until_4m(b: &mut Bencher) {
    let max_fib_number = test::black_box(4_000_000);

    b.iter(|| {
        let res: u32 = Fib::new()
            .take_while(|x| x < &max_fib_number)
            .filter(|x| x % 2 == 0)
            .sum();
        assert_eq!(res, 4613732);
    });
}
