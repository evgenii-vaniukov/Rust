use std::io;
use std::str::FromStr;

fn needed_sum(n1: usize, v1: &Vec<i32>, n2: usize, v2: &mut Vec<i32>, k: i32) -> u32 {

    let mut result: u32 = 0; 
    //v2.reverse();
    for i in 0..n1 {
        for j in 0..n2{
            if !(v1[i] + v2[j] != k) {
                result += 1
            }
        }
    }
    return result
}

fn main() {
    //println!("Please, type an input");

    let input = read_input::<i32>();

    let n1: usize = input.0;
    let v1: Vec<i32> = input.1.unwrap();
    let n2: usize = input.2;
    let mut v2: Vec<i32> = input.3.unwrap();
    let k: i32 = input.4;

    println!("{}", needed_sum(n1, &v1, n2, &mut v2, k));
}

fn read_input<T: FromStr>() -> (usize, Result<Vec<T>, T::Err>, usize, Result<Vec<T>, T::Err>, i32) {
    
    let mut n1: String = String::new();
    io::stdin().read_line(&mut n1).expect("could not read from stdin");
    let n1 = n1.trim().parse::<usize>().expect("Type an integer");


    let mut v1: String = String::new();
    io::stdin().read_line(&mut v1).expect("could not read from stdin");
    let v1 = v1.trim().split_whitespace().map(|word| word.parse()).collect();


    let mut n2: String = String::new();
    io::stdin().read_line(&mut n2).expect("could not read from stdin");
    let n2 = n2.trim().parse::<usize>().expect("Type an integer");

    let mut v2: String = String::new();
    io::stdin().read_line(&mut v2).expect("could not read from stdin");
    let v2 = v2.trim().split_whitespace().map(|word| word.parse()).collect();

    let mut k: String = String::new();
    io::stdin().read_line(&mut k).expect("could not read from stdin");
    let k = k.trim().parse::<i32>().expect("Type an integer");
    

    return (n1, v1, n2, v2, k)
}