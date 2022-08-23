use std::io;
//use regex::Regex;

fn main() {
    let input = read_input();
     

    //println!("{:?}", conversion(input));
    println!("{:?}", calculation(conversion(input)));
    
}


fn read_input() -> Vec<String> {

    let mut s: String = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read from stdin");
    let s: Vec<String>= s.trim().split(' ').map(|word| word.into()).collect();


    return s
}

fn conversion(v: Vec<String>) -> Vec<String> {
    
    let mut stack: Vec<String> = Vec::new(); 
    let mut reverse_polish_notation: Vec<String> = Vec::new();

    for i in v {
        match i.as_str() {
            "-" => { if stack.is_empty() {                                                                     
                            stack.push(i.to_string());
                        }
                    else if stack.last().unwrap() == "(" {   
                        stack.push(i.to_string()); 
                        }
                    else {
                        while !(stack.is_empty()) {
                            if stack.last().unwrap() == "(" { break; }
                            else {
                                reverse_polish_notation.push(stack.pop().unwrap());}
                        }
                        stack.push(i.to_string());
                    }
                },   
            "+" => { if stack.is_empty() {                                                                     
                            stack.push(i.to_string());
                        }
                    else if stack.last().unwrap() == "(" { 
                            stack.push(i.to_string()); }
                    else {
                        while !(stack.is_empty()) {
                            if stack.last().unwrap() == "(" { break; }
                            else {
                                reverse_polish_notation.push(stack.pop().unwrap());}
                        }
                        stack.push(i.to_string());
                    }
                },   
            "*" => { if stack.is_empty() {                                                                     
                            stack.push(i.to_string());} 
                    else if stack.last().unwrap() == "(" { 
                            stack.push(i.to_string()); }
                    else {
                        while !stack.is_empty() && (stack.last().unwrap() == "*" || stack.last().unwrap() == "/" ) {
                            if stack.last().unwrap() == "("  {break;}
                            else {
                                reverse_polish_notation.push(stack.pop().unwrap());
                            }
                        }
                        stack.push(i.to_string());
                    }
                    
                },
            "/" => {if stack.is_empty() {                                                                     
                            stack.push(i.to_string());} 
                    else if stack.last().unwrap() == "(" { 
                            stack.push(i.to_string()); }
                    else {
                        while !stack.is_empty() && (stack.last().unwrap() == "*" || stack.last().unwrap() == "/" )  {
                            if stack.last().unwrap() == "(" {break;}
                            else {
                                reverse_polish_notation.push(stack.pop().unwrap());
                            }
                        }
                        stack.push(i.to_string());
                    }
                },
            "(" => { stack.push(i.to_string());
                },
            ")" => { let mut l = stack.pop().unwrap();
                    while l != "(" {
                        reverse_polish_notation.push(l);
                        l = stack.pop().unwrap();
                    }
                },
            _ => {reverse_polish_notation.push(i.to_string());},
            }
    }
    stack.reverse();
    reverse_polish_notation.append(&mut stack);
    
    return reverse_polish_notation  
}


fn calculation(v: Vec<String>) -> i32 {

    let mut stack: Vec<i32> = Vec::new();

    for i in v {

        match i.as_str() {

            "-" => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a - b);
            },
            "+" => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a + b);
            },

            "/" => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a / b);
            },

            "*" => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a * b);
            },

            "\n" => {}, 

            _ => {stack.push(i.parse::<i32>().unwrap());
            },
        }
    }
    return stack.pop().unwrap()


}
