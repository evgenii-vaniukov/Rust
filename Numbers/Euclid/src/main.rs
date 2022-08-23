use std::io;

fn main() {
    println!("Please, type first number");

    let mut a: String = String::new();

    io::stdin()
        .read_line(&mut a)
        .expect("Failed");

    let a: i32 = a.trim().parse::<i32>().expect("Type an integer"); 

    println!("Please, type second number");

    let mut b: String = String::new();

    io::stdin()
        .read_line(&mut b)
        .expect("Failed");

    let b: i32 = b.trim().parse::<i32>().expect("Type an integer"); 

    println!("The Greatest Common Devisor is: {:?}", euclid(a,b));
    println!("The Lowest Common Multiple is: {}", LCM(a,b));
}

fn euclid(mut a: i32, mut b: i32) -> (i32, i32,i32) {
    let first: i32 = a;
    let second: i32 = b;

    while b != 0 {
        let r = a % b;
        a = b ;
        b = r;
    }
    return (first, second, a)
}

fn LCM(a: i32, b: i32) -> i32 {
    return a * b / euclid(a,b).2
}