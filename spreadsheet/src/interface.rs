use crate::parser::command::Command;
use crate::parser::print_output::print_sheet;
use crate::spreadsheet::SpreadSheet;

pub fn process_command(command: &Command, spreadsheet: &mut SpreadSheet, row: &mut usize, col: &mut usize, enable_output: &mut bool, quit: &mut bool, max_rows: &usize, max_cols: &usize) {
    match command {
        Command::RangeCommand(cmd) => {

        },
        Command::ArithmeticCommand(cmd) => {

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
                },
                _ => ()
            }
        }
    }
}
