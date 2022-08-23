use std::io;

fn main() {
    println!("Please, type a number");

    let mut a: String = String::new();
    
    io::stdin()
        .read_line(&mut a)
        .expect("Failed");

    let a: i32 = a.trim().parse::<i32>().expect("Type an integer");

    println!("Please, type a power");

    let mut n: String = String::new();
    
    io::stdin()
        .read_line(&mut n)
        .expect("Failed");

    let n: u32 = n.trim().parse::<u32>().expect("Type an integer");

    println!("{}, {}", a, n);

    println!("{:?}", power(a, n));

}

fn power(a: i32, mut n: u32) -> (i32, i32, u32) {
    
    let mut a_in_power_of2 = a;
    let mut result = 1; 
    
    let power = n;

 
    while n > 0 {
        if n % 2 == 1 {
            result *= a_in_power_of2;
        }

        a_in_power_of2 *= a_in_power_of2;

        n /= 2;
    }
    return (result.try_into().unwrap(), a, power)
}
