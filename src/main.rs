use core::panic;
use std::{io, usize};

fn main() {
    println!("Hello, world!");

    let mut expression = String::new();
    io::stdin().read_line(&mut expression).unwrap();
    
    println!("Result: {}", eval(&expression.trim()));
}

fn eval(exp: &str) -> i32 {
    let contents = split_according_to_precedence(&exp);

    match contents {
        Some(expression) => {
            println!("{:?}", expression);
            match expression.1 {
                '+' => eval(expression.0) + eval(expression.2),
                '-' => eval(expression.0) - eval(expression.2),
                '*' => eval(expression.0) * eval(expression.2),
                '/' => eval(expression.0) / eval(expression.2),
                _ => panic!("NAH"),
            }
        }

        None => {
            exp.replace("(", "").replace(")", "").trim().parse::<i32>().expect("nah")
        }
    }

}

fn precedence(op: char) -> usize {
    match op {
        '+' | '-' => 1,
        '*' | '/' => 2,
        _ => 0,
    }
}

fn split_according_to_precedence(exp: &str) -> Option<(&str, char, &str)> {
    let mut parantheses_count = 0;
    let mut last_split_idx = None;
    let mut min_precedence = usize::MAX;

    for (idx, c) in exp.chars().enumerate() {
        match c {
            '(' => parantheses_count += 1,
            ')' => parantheses_count -= 1,
            '+' | '-' | '*' | '/' if parantheses_count == 0 => {
                let current_precedence = precedence(c);

                if current_precedence <= min_precedence {
                    last_split_idx = Some(idx);
                    min_precedence = current_precedence;
                }
            }
            _ => {}
        }
    }

    if let Some(split_idx) = last_split_idx {
        let lhs = &exp[..split_idx].trim(); 
        let op = exp.chars().nth(split_idx).unwrap(); 
        let rhs = &exp[split_idx + 1..].trim(); 
        return Some((lhs, op, rhs));
    }

    if exp.starts_with('(') && exp.ends_with(')') {
        return split_according_to_precedence(&exp[1..exp.len() - 1]);
    }

    None 
}
