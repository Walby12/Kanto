use std::fs;
use std::env;
use std::io;

fn tokenize(source_code: &str) {
    let mut stack: Vec<i32> = Vec::new();
    let mut i = 1;

    for token in source_code.split_whitespace() {
        match token {
            "+" => {
                if stack.len() < 2 {
                    println!("Error: Not enough values on stack for '+' at index {}", i);
                    return;
                }
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a + b);
            },
            ";" => {
                if stack.len() < 1 {
                    println!("Error: Not enough values on stack for ';' at idnex {}", i);
                }
                let a = stack.pop().unwrap();
                println!("{}", a);
            },
            "-" => {
                if stack.len() < 2 {
                    println!("Error: Not enough values on stack for '-' at index {}", i);
                    return;
                }
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(b - a);
            },
            "*" => {
                if stack.len() < 2 {
                    println!("Error: not enough values on stack for '*' at index {}", i);
                    return;
                }
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a * b);
            },
            "/" => {
                if stack.len() < 2 {
                    println!("Error: not enough values on stack for '/' at index {}", i);
                    return;
                }
                let a = stack.pop().unwrap();
                if a == 0 {
                    println!("Error: division by zero at index {}", i)
                }
                let b = stack.pop().unwrap();
                stack.push(b / a);
            },
            "dup" => {
                if stack.len() < 1 {
                    println!("Error: not enough values on stack for 'dup' at index {}", i);
                    return;
                }
                let a = stack.last().copied().unwrap();
                stack.push(a);
            },
            "swap" => {
                if stack.len() < 2 {
                    println!("Error: not enough values on stack for 'swap' at index {}", i);
                    return;
                }
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(b);
                stack.push(a);
            },
            "drop" => {
                if stack.len() < 1 {
                    println!("Error: not enough values on stack for 'drop' at index {}", i);
                    return;    
                }
                let a = stack.pop().unwrap();
                println!("Droppped: {}", a);
            },
            "over" => {
                if stack.len() < 2 {
                    println!("Error: not enough values on stack for 'over' at index {}", i);
                    return;
                }
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(b);
                stack.push(a);
                stack.push(b);
            },
            "show" => {
                println!("Stack ;---;");
                for i in stack.clone() {
                    print!("{}  ", i);
                }
            },
            _ => {
                match token.parse::<i32>() {
                    Ok(num) => stack.push(num),
                    Err(_) => {
                        println!("Error: Invalid token '{}' at index {}" , token, i);
                        return;
                    }
                }
            }
        }
        i += 1;
    }
}

fn load_prog_from_mem(path: String) {
    match fs::read_to_string(path) {
        Ok(contents) => {
            tokenize(contents.as_str());
        }
        Err(e) => {
            println!("Failed to load file from memory {}", e);
        }
    }
}

fn usage() {
    println!("USAGE: kanto [file_path] -- sims the prog");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Too many args");
        usage();
    } else if args.len() == 1 {
        usage();
    } else {
        load_prog_from_mem(args[1].clone());
    }
}

