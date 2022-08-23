use std::io;

fn main() {
    println!("Please, type a number");
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed");

    let n: u32 = n.trim().parse::<u32>().expect("Input an integer");
    println!("The value of n fibonacci is {}", fibonacci(n));
}

fn fibonacci(n: u32) -> u32 {
    if n == 0 || n == 1 {return 1}

    let mut current = 1; 
    let mut prev = 1;
    for _ in 2..=n {
        let temp = current;
        current += prev;
        prev = temp;
        println!("{}", current)
    }
    return current
}