// Найти, на сколько нулей оканчивается n! = 1 · 2 · 3 · … · n. n ≤ 1000

use std::io;

fn main() {
    println!("Please, type a number");

    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed");

    let n: u16 = n.trim().parse::<u16>().expect("Plese, type an integer");

    println!("{}", factorial1(n));
    //println!("{}", n) OWnership is not transfered;

}

// Данное решение не умеет работать с большими числами
fn factorial(n: u128) -> u32 {
    let mut result: u128 = 1;
    (2..=n).for_each(|i| {
        result *= i;
    });

    let mut counter: u32 = 0;

    while result % 10 == 0 {
        counter += 1;
        result /= 10;
    }
    return counter
}
// Верное решение
fn factorial1(mut n: u16) -> u16 {
    let mut counter: u16 = 0;

    while n > 0 {
        n /= 5;
        counter += n;
    }
    return counter
}



