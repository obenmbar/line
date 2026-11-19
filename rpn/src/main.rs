 use std::env;
fn main() {
 let args : Vec<_> =  env::args().collect();
 if args.len() != 2 {
    println!("Error");
    return ;
 }

  let mut stack: Vec<i64> = Vec::new();
  for val in  args[1].split_whitespace(){
    if let Ok(num) = val.parse::<i64>() {
        stack.push(num);
    }else {
        if stack.len() < 2{
            println!("Error");
            return ;
        }

        let b =  stack.pop().unwrap();
        let a = stack.pop().unwrap();

        let res = match val {
            "+" => a+b,
            "-" => a-b ,
            "*" => a*b,
            "/" => {
                if b == 0 {
                    println!("Error");
                     return ;
                }
                a/b
            } ,
            "%" => { if b == 0 {
                    println!("Error");
                     return ;
                }
                a%b} ,
            _ => {
                println!("Error");
                     return ;
            },
        };

        stack.push(res);
    }
  }
  if stack.len()!= 1 {
     println!("Error");
     return ;
  }
 println!("{}", stack[0])
}