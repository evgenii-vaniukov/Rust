use std::io;
use std::str::FromStr;

fn main() {
    //println!("Please, type an input");

    let input = read_input::<i32>();

    let n1: usize = input.0;
    let v1: Vec<i32> = input.1.unwrap();
    let n2: usize = input.2;
    let v2: Vec<i32> = input.3.unwrap();


    //println!("{},{}", n1,n2);
    //println!("{:?}", v1);
    //println!("{:?}", v2);

    let result = find_the_closest(n1, &v1, n2, &v2);
    result.iter().for_each(|val| print!("{} ", val));
}


fn read_input<T: FromStr>() -> (usize, Result<Vec<T>, T::Err>, usize, Result<Vec<T>, T::Err>) {
    
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
    

    return (n1, v1, n2, v2)
}


fn find_the_closest(n1: usize, v1: &Vec<i32>, n2: usize, v2: &Vec<i32>) -> Vec<usize> {

    let mut result: Vec<usize> = Vec::new(); 

    for i in 0..n2 {
        let mut diff: Vec<i32> = Vec::new();
        for j in 0..n1{
            diff.push((v2[i] - v1[j]).abs())
        }
        
        let min_value = diff.iter().min();

        match min_value {
            None => println!(""), 
            Some(i) => {let index_element = diff
                            .iter()
                            .position(|&x| x == *min_value.unwrap())
                            .unwrap();
                        result.push(index_element);},
        }

        }
    return result
}