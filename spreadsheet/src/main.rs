mod utils;
mod operand;
mod equation;
mod parser;
mod spreadsheet;
mod interface;

use std::env;
use std::io;
use spreadsheet::SpreadSheet;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        println!("Expected two argument.");
        std::process::exit(1);
    }
    
    let m: usize = match args[1].parse() {
        Ok(m) => m,
        Err(_) => {
            println!("Invalid usize argument: {}", args[1]);
            std::process::exit(1);
        }
    };

    let n: usize = match args[2].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid usize argument: {}", args[1]);
            std::process::exit(1);
        }
    };

    let mut spreadsheet = SpreadSheet::new(m,n);

    parser::print_output::print_sheet(1, 1, m, n);
    
    let mut enable_output = true;
    let mut row = 1;
    let mut col = 1;
    let mut quit = false;
    'user_interaction: loop {
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read user input");
        let user_input = user_input.trim();
        let command: Result<parser::command::Command, parser::error::Error> = parser::parser::parse_cmd(user_input);
        match command {
            Ok(command) => {
                interface::process_command(&command, &mut spreadsheet, &mut row, &mut col, &mut enable_output, &mut quit, &m, &n);
                if quit {
                    break 'user_interaction;
                }
            },
            Err(_) => {
                
            }
        }
    }
}