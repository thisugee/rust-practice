pub fn fib(n: u32) -> u32 {
    if n <= 1 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

#[test]
fn fib_test() {
    assert_eq!(1, fib(2));
    assert_eq!(2, fib(3));
    assert_eq!(5, fib(5));
    assert_eq!(34, fib(9));
}
