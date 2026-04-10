fn main() {
    println!("Hello, world!");
    println!("Fibonacci sequence:");
    for i in 0..10 {
        println!("{}: {}", i, fibonacci(i));
    }
}

fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}
