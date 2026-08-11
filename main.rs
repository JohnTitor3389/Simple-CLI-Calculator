use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Num {
    #[arg(short = 'x', long)]
    number_1: f64,
    #[arg(short = 'y', long)]
    number_2: f64,
    #[command(subcommand)]
    operation: Operation,
}

#[derive(Subcommand)]
enum Operation {
    Add,
    Sub,
    Mult,
    Div,
}

fn div(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Can't divide by 0".to_string())
    }
    else {
        Ok(a / b)
    }
}

fn main() {
    let num = Num::parse();
    
    let result = match num.operation {
       Operation::Add => Ok(num.number_1 + num.number_2),
       Operation::Sub => Ok(num.number_1 - num.number_2),
       Operation::Mult => Ok(num.number_1 * num.number_2),
       Operation::Div => div(num.number_1, num.number_2),
    };
       
    match result {
        Ok(num) => println!("{}", num),
        Err(e) => println!("{}", e),    
    };
}