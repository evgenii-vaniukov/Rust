use std::io;
use std::str::FromStr;

fn main() {

    let mut v: Vec<i32> = read_to_vec::<i32>().unwrap();
    v.reverse();
    v.iter().for_each(|val| print!("{} ", val));

    
}

fn read_to_vec<T: FromStr>() -> Result<Vec<T>, T::Err> {
    let mut s1: String = String::new(); 
    let mut s2: String = String::new();

    io::stdin().read_line(&mut s1).expect("could not read from stdin");
    io::stdin().read_line(&mut s2).expect("could not read from stdin");

    s2.trim().split_whitespace().map(|word| word.parse()).collect()
}

