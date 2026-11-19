use std::env;

fn main() {
     let args Vec<String> = env::args().collect();
     if args.len() != 2 {
        println!("ERROR");
        return ;
     }

    let mut stack: Vec<String> = Vec::new();
    
    for tocken in args[1].split_whitespace() {
        if let OK(num) = tocken.parse::<i64>() {
            stack.push(num)
        }else {
            if stack.len() < 2 {
                println!("ERROR");
                return;
            }

             let b =  stack.pop().unwrap();
             let a =  stack.pop().unwrap();

             match tocken {
                "+" => a+b,
                "-" => a-b,
                "*" => a*b,
                "/" => {
                    if b == 0 {
                         println!("ERROR");
                           return;
                    }
                    a/b
                }
                 "%" => {
                     if b == 0 {
                        println!("ERROR");
                        return;
                     }
                     a%b 
                 }
             }
        }
    }
}