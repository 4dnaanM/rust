mod equation;
mod interface;
mod parser;
mod spreadsheet;
mod utils;
mod value;
mod vcs;

use spreadsheet::SpreadSheet;
use crate::vcs::vcs::VersionControlSystem;
use std::env;
use std::io;
use std::io::Write;

pub fn main() {
    let _args: Vec<String> = env::args().collect();

    // let args= [String::from(""),String::from("10"),String::from("10")];

    let m : usize = 999;
    let n : usize = 999;

    let mut spreadsheet = SpreadSheet::new(m, n);

    parser::print_output::print_sheet(1, 1, &spreadsheet, m, n);
    print!("[0.0] (ok) > ");
    // let mut enable_output = true;
    // let mut row = 1;
    // let mut col = 1;
    // let mut quit = false;
    
    let mut vcs = VersionControlSystem::new("./vcs");
    vcs.commit(&spreadsheet, "Initial commit");
    vcs.list();
    spreadsheet = vcs.checkout(1, &spreadsheet);
    vcs.commit(&spreadsheet, "Second commit");
    vcs.list();
    spreadsheet = vcs.checkout(2, &spreadsheet);
    vcs.commit(&spreadsheet, "Third commit");
    vcs.list();
    io::stdout().flush().unwrap();
    
    // 'user_interaction: loop {
    //     let mut user_input = String::new();
    //     io::stdin()
    //         .read_line(&mut user_input)
    //         .expect("Failed to read user input");
    //     let user_input = user_input.trim();
    //     interface::process_command(
    //         user_input,
    //         &mut spreadsheet,
    //         &mut row,
    //         &mut col,
    //         &mut enable_output,
    //         &mut quit,
    //         &m,
    //         &n,
    //     );
    //     if quit {
    //         break 'user_interaction;
    //     }
    // }
}
