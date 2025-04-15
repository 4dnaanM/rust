use crate::parser;
use crate::parser::command::Command;
use crate::parser::error::Error;
use crate::parser::print_output::print_sheet;
use crate::spreadsheet::SpreadSheet;
use std::time::Instant;
use std::io::{self, Write};

pub fn process_command(user_input: &str, spreadsheet: &mut SpreadSheet, row: &mut usize, col: &mut usize, enable_output: &mut bool, quit: &mut bool, max_rows: &usize, max_cols: &usize) {
    let start = Instant::now();
    let user_command: Result<Command, Error> = parser::parser::parse_cmd(user_input);
    let Ok(command) = user_command else {
        let duration = start.elapsed();
        print!("[{:.1}] (invalid command) > ", duration.as_secs_f64());
        io::stdout().flush().unwrap();
        return
    };
    match command {
        Command::RangeCommand(cmd) => {

        },
        Command::ArithmeticCommand(cmd) => {
            std::thread::sleep(std::time::Duration::from_secs(2));
        },
        Command::UserInteractionCommand(cmd) => {
            let ui_command = cmd.command.clone();
            match ui_command.as_str() {
                "enable_output" => {
                    *enable_output = true;
                },
                "disable_output" => {
                    *enable_output = false;
                }
                "w" => {
                    if *enable_output {
                        if *row > 10 {
                            *row = (*row) - 10;
                        } else {
                            *row = 1;
                        }
                        print_sheet(*row , *col, *max_rows, *max_cols);
                    }
                },
                "a" => {
                    if *enable_output {
                        if *col > 10 {
                            *col = (*col) - 10;
                        } else {
                            *col = 1;
                        };
                        print_sheet(*row , *col, *max_rows, *max_cols);
                    }
                },
                "s" => {
                    if *enable_output {
                        *row = ((*row) + 10).min(*max_rows);
                        print_sheet(*row , *col, *max_rows, *max_cols);
                    }
                },
                "d" => {
                    if *enable_output {
                        *col = ((*col) + 10).min(*max_cols);
                        print_sheet(*row , *col, *max_rows, *max_cols);
                    }
                },
                "q" => {
                    *quit = true;
                    return
                },
                _ => ()
            }
        }
    }
    let duration = start.elapsed();
    print!("[{:.1}] (ok) > ", duration.as_secs_f64());
    io::stdout().flush().unwrap();
}
