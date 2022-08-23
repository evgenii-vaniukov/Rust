use std::io;

fn main() {
    println!("Please, type a number:");
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed");
    let n: i32 = n.trim().parse::<i32>().expect("Type a number");
    
    if is_prime(&n) {
        println!("The number is prime");
    } else {
        println!("THe number is not prime");
    }
}

fn is_prime(n: &i32) -> bool {
    
    const ONE: i32 = 1;
    if n == &ONE {return false}

    let mut k = 2;

    while &(k * k) <= n {
        if n % k == 0 {return false}
        k += 1
    }
    return true
}