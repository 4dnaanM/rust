mod equation;
mod interface;
mod parser;
mod spreadsheet;
mod utils;
mod value;
mod vcs;

use spreadsheet::SpreadSheet;
use std::env;
use std::io;
use std::io::Write;

pub fn main() {
    let args: Vec<String> = env::args().collect();

    // let args= [String::from(""),String::from("10"),String::from("10")];

    if args.len() != 3 {
        println!("Expected two arguments");
        std::process::exit(1);
    }

    let (m, n, vcs) = if args[1] == "vcs-dir" {
        // args[2] is dir. Assuming that it is synatctically correct.
        // Correction check to be added later.

        let vcs_dir = &args[2];
        let serial_vcs = vcs::vcs_engine::SerialVcs::load(vcs_dir);
        let vcs = vcs::vcs_engine::VersionControl::load(serial_vcs, vcs_dir.clone());

        let (m, n) = vcs.get_m_n();
        (m, n, vcs)
    } else {
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

        if m == 0 || m > 999 || n == 0 || n > 18278 {
            println!(
                "Invalid m or n value. m should be between 1 and 999, n should be between 1 and 18278."
            );
            std::process::exit(1);
        }

        let vcs_dir = "./vcs_dir".to_string();
        let vcs = vcs::vcs_engine::VersionControl::new(vcs_dir, &m, &n);

        (m, n, vcs)
    };

    let mut vcs = vcs;
    let mut spreadsheet = SpreadSheet::new(m, n);

    parser::print_output::print_sheet(1, 1, &spreadsheet, m, n);
    print!("[0.0] (ok) > ");
    io::stdout().flush().unwrap();
    let mut enable_output = true;
    let mut row = 1;
    let mut col = 1;
    let mut quit = false;
    'user_interaction: loop {
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read user input");
        let user_input = user_input.trim();
        interface::process_command(
            user_input,
            &mut spreadsheet,
            &mut vcs,
            &mut row,
            &mut col,
            &mut enable_output,
            &mut quit,
        );
        if quit {
            break 'user_interaction;
        }
    }
}
