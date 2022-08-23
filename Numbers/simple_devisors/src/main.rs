// Вывести разложение натурального числа n на простые множители.
// Простые множители должны быть упорядочены по возрастанию и разделены пробелами.

use std::io;

fn main() {
    println!("Please, type a number");

    let mut n: String = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed");

    let n: u32 = n.trim().parse::<u32>().expect("Please, type an integer");
    simple_divisors(n).iter().for_each(|val| print!("{} ", val));
}

fn simple_divisors(mut n: u32) -> Vec<u32> {
    
    let mut v = Vec::new();

    let mut div: u32 = 2; 

    while n > 1 {
        while n % div == 0 {
            n /= div;
            v.push(div);
        }
        div += 1;
    }
    v.sort();
    return v
}


