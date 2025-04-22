use crate::parser;
use crate::parser::cell::Value;
use crate::parser::command::Command;
use crate::parser::error::Error;
use crate::parser::print_output::print_sheet;
use crate::spreadsheet::SpreadSheet;
use crate::utils::{Status, Type};
use std::io::{self, Write};
use std::time::Instant;
use crate::vcs::vcs::VersionControlSystem;

pub fn process_command(
    user_input: &str,
    spreadsheet: &mut SpreadSheet,
    vcs: &mut VersionControlSystem,
    row: &mut usize,
    col: &mut usize,
    enable_output: &mut bool,
    quit: &mut bool,
    max_rows: &usize,
    max_cols: &usize,
) {
    let start = Instant::now();
    let user_command: Result<Command, Error> =
        parser::parser::parse_cmd(user_input, *max_rows, *max_cols);
    let Ok(command) = user_command else {
        let duration = start.elapsed();
        if *enable_output {
            print_sheet(1, 1, spreadsheet, *max_rows, *max_cols);
        }
        print!("[{:.1}] (invalid command) > ", duration.as_secs_f64());
        io::stdout().flush().unwrap();
        return;
    };
    let status;
    match command {
        Command::RangeCommand(cmd) => {
            let Value::Cell(cell) = cmd.target_cell else {
                panic!();
            };
            let Value::Cell(operand_1) = cmd.operand_1 else {
                panic!();
            };
            let Value::Cell(operand_2) = cmd.operand_2 else {
                panic!();
            };
            let t = Type::from_str(cmd.function.as_str());
            status = match spreadsheet.set_cell_equation(
                cell.row - 1,
                cell.col - 1,
                Some((operand_1.row - 1, operand_1.col - 1)),
                Some((operand_2.row - 1, operand_2.col - 1)),
                None,
                None,
                t,
            ) {
                Status::OK => true,
                Status::ERR => false,
            };

            if *enable_output {
                print_sheet(1, 1, spreadsheet, *max_rows, *max_cols);
            }
        }
        Command::ArithmeticCommand(cmd) => {
            let Value::Cell(cell) = cmd.target_cell else {
                panic!();
            };

            let (cell_1, const_1) = match cmd.operand_1 {
                Value::Cell(cell) => (Some((cell.row - 1, cell.col - 1)), None),
                Value::Constant(constant) => (None, Some(constant)),
            };

            let (cell_2, const_2) = match cmd.operand_2 {
                Some(operand_2) => match operand_2 {
                    Value::Cell(cell_2) => (Some((cell_2.row - 1, cell_2.col - 1)), None),
                    Value::Constant(constant) => (None, Some(constant)),
                },
                None => (None, None),
            };

            let t = match cmd.operator {
                Some(op) => Type::from_str(op.as_str()),
                None => Type::from_str("+"),
            };

            status = match spreadsheet.set_cell_equation(
                cell.row - 1,
                cell.col - 1,
                cell_1,
                cell_2,
                const_1,
                const_2,
                t,
            ) {
                Status::OK => true,
                Status::ERR => false,
            };

            if *enable_output {
                print_sheet(1, 1, spreadsheet, *max_rows, *max_cols);
            }
        }
        Command::UserInteractionCommand(cmd) => {
            let ui_command = cmd.command.clone();
            match ui_command.as_str() {
                "enable_output" => {
                    *enable_output = true;
                    print_sheet(*row, *col, spreadsheet, *max_rows, *max_cols);
                }
                "disable_output" => {
                    *enable_output = false;
                }
                "w" => {
                    if *enable_output {
                        if *row > 10 {
                            *row -= 10;
                        } else {
                            *row = 1;
                        }
                        print_sheet(*row, *col, spreadsheet, *max_rows, *max_cols);
                    }
                }
                "a" => {
                    if *enable_output {
                        if *col > 10 {
                            *col -= 10;
                        } else {
                            *col = 1;
                        };
                        print_sheet(*row, *col, spreadsheet, *max_rows, *max_cols);
                    }
                }
                "s" => {
                    if *enable_output {
                        *row = ((*row) + 10).min(*max_rows - 9);
                        print_sheet(*row, *col, spreadsheet, *max_rows, *max_cols);
                    }
                }
                "d" => {
                    if *enable_output {
                        *col = ((*col) + 10).min(*max_cols - 9);
                        print_sheet(*row, *col, spreadsheet, *max_rows, *max_cols);
                    }
                }
                "q" => {
                    *quit = true;
                    return;
                }
                "scroll_to" => {
                    if let Some(scroll_to_cell) = cmd.scroll_to_cell {
                        *row = scroll_to_cell.row;
                        *col = scroll_to_cell.col;
                        if *enable_output {
                            print_sheet(*row, *col, spreadsheet, *max_rows, *max_cols);
                        }
                    };
                }
                _ => (),
            }
            status = true;
        }
        Command::SleepCommand(cmd) => {
            let Value::Cell(target_cell) = cmd.target_cell else {
                panic!();
            };

            let (cell_1, const_1) = match cmd.value {
                Value::Cell(cell) => (Some((cell.row - 1, cell.col - 1)), None),
                Value::Constant(constant) => (None, Some(constant)),
            };

            let t = Type::SLP;

            status = match spreadsheet.set_cell_equation(
                target_cell.row - 1,
                target_cell.col - 1,
                cell_1,
                None,
                const_1,
                None,
                t,
            ) {
                Status::OK => true,
                Status::ERR => false,
            };

            if *enable_output {
                print_sheet(1, 1, spreadsheet, *max_rows, *max_cols);
            }
        }
        Command::VCSCommand(cmd) => {
            status = true;
            let command = cmd.command.clone();
            let command = command.as_str();
            match command {
                "list" => {
                    vcs.list();
                },
                "commit" => {
                    if let Some(argument) = cmd.argument {
                        vcs.commit(spreadsheet, &argument);
                    }
                },
                "checkout" => {
                    if let Some(argument) = cmd.argument {
                        match argument[0..].parse::<usize>() {
                            Ok(commit_id) => {
                                *spreadsheet = vcs.checkout(commit_id as u32, spreadsheet);
                            }
                            Err(_) => ()
                        }
                        if *enable_output {
                            print_sheet(1, 1, spreadsheet, *max_rows, *max_cols);
                        }            
                    }
                },
                _ => ()
            }
        }
    }
    let duration = start.elapsed();
    if status {
        print!("[{:.1}] (ok) > ", duration.as_secs_f64());
    } else {
        print!("[{:.1}] (err) > ", duration.as_secs_f64());
    }
    io::stdout().flush().unwrap();
}
