use std::time::Instant;

fn main() {
    let n = 50;

    println!("Hello, world! Fib tests for sample size {}", n);

    let start = Instant::now();
    println!("Fib: {}", fib(n));
    let duration = start.elapsed();
    println!("-----> Fib performance: {:?}", duration);

    let start = Instant::now();
    println!("Fib 2: {}", fib2(n));
    let duration = start.elapsed();
    println!("-----> Fib 2 performance: {:?}", duration);
}

fn fib(n: i64) -> i64 {
    if n == 0 { return 0 };
    if n == 1 { return 1 };
    return fib(n-1) + fib(n-2)
}

fn fib2(n: i64) -> i64 {
    fn fib2_inner(a: i64, b: i64, n: i64) -> i64 {
        if n == 0 { return a };
        return fib2_inner(b, a + b, n - 1)
    }
    return fib2_inner(0, 1, n)
}
