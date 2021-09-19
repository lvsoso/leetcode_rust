
// n! n*(n-1)*(n-2)...
fn factorial(n:usize) -> usize {
    if n <= 1 {
        return 1;
    }

    return n * factorial(n - 1 );
}


fn fibonacci(n: usize) -> usize {
    if n == 0 {
        return 0;
    }

    if  n == 1 {
        return 1;
    }
    return fibonacci(n - 1) + fibonacci(n - 2)
}

fn main() {
    println!("{}", factorial(5));
    println!("{}", fibonacci(5));
}
