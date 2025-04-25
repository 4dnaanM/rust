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

    let (m, n, vcs) = if args.len() == 3 {
        // Normal mode: cargo run -- 10 10
        let m: usize = args[1].parse().unwrap_or_else(|_| {
            println!("Invalid m argument: {}", args[1]);
            std::process::exit(1);
        });

        let n: usize = args[2].parse().unwrap_or_else(|_| {
            println!("Invalid n argument: {}", args[2]);
            std::process::exit(1);
        });

        if m == 0 || m > 999 || n == 0 || n > 18278 {
            println!("Invalid m or n value. m should be between 1 and 999, n between 1 and 18278.");
            std::process::exit(1);
        }

        let vcs = vcs::vcs_engine::VersionControl::dummy();
        (m, n, vcs)
    } else if args.len() >= 2 && args[1] == "--vcs" {
        let mut vcs_dir = None;
        let mut rows = None;
        let mut cols = None;

        let mut i = 2;
        while i < args.len() {
            match args[i].as_str() {
                "--vcs_dir" => {
                    i += 1;
                    vcs_dir = Some(args.get(i).cloned().unwrap_or_else(|| {
                        println!("Missing value for --vcs_dir");
                        std::process::exit(1);
                    }));
                }
                "--rows" => {
                    i += 1;
                    rows = Some(
                        args.get(i)
                            .and_then(|s| s.parse::<usize>().ok())
                            .unwrap_or_else(|| {
                                println!("Invalid or missing value for --rows");
                                std::process::exit(1);
                            }),
                    );
                }
                "--cols" => {
                    i += 1;
                    cols = Some(
                        args.get(i)
                            .and_then(|s| s.parse::<usize>().ok())
                            .unwrap_or_else(|| {
                                println!("Invalid or missing value for --cols");
                                std::process::exit(1);
                            }),
                    );
                }
                _ => {
                    println!("Unknown argument: {}", args[i]);
                    std::process::exit(1);
                }
            }
            i += 1;
        }

        if let Some(dir) = vcs_dir {
            let serial_vcs = vcs::vcs_engine::SerialVcs::load(&dir);
            let vcs = vcs::vcs_engine::VersionControl::load(serial_vcs, dir.clone());
            let (m, n) = vcs.get_m_n();
            (m, n, vcs)
        } else if let (Some(m), Some(n)) = (rows, cols) {
            if m == 0 || m > 999 || n == 0 || n > 18278 {
                println!(
                    "Invalid m or n value. m should be between 1 and 999, n between 1 and 18278."
                );
                std::process::exit(1);
            }
            let vcs_dir = "./vcs_dir".to_string();
            let vcs = vcs::vcs_engine::VersionControl::new(vcs_dir, &m, &n);
            (m, n, vcs)
        } else {
            println!("Provide either --vcs_dir or both --rows and --cols");
            std::process::exit(1);
        }
    } else {
        println!(
            "Invalid arguments. Use either `cargo run -- m n` or `cargo run -- --vcs [--vcs_dir path | --rows m --cols n]`."
        );
        std::process::exit(1);
    };

    let mut vcs = vcs;
    let mut spreadsheet = SpreadSheet::new(m, n);
    if vcs.get_m_n() != (0, 0) {
        vcs.commit("Initial_commit", &mut spreadsheet);
    }

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
