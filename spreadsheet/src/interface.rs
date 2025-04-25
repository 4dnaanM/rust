use crate::parser;
use crate::parser::cell::Value;
use crate::parser::command::Command;
use crate::parser::error::Error;
use crate::parser::print_output::print_sheet;
use crate::spreadsheet::SpreadSheet;
use crate::utils::{Status, Type};
use crate::vcs::vcs_engine::VersionControl;
use std::io::{self, Write};
use std::time::Instant;

pub fn process_command(
    user_input: &str,
    spreadsheet: &mut SpreadSheet,
    vcs: &mut VersionControl,
    row: &mut usize,
    col: &mut usize,
    enable_output: &mut bool,
    quit: &mut bool,
) {
    let max_rows = spreadsheet.m;
    let max_cols = spreadsheet.n;
    let start = Instant::now();
    let user_command: Result<Command, Error> =
        parser::command_parser::parse_cmd(user_input, max_rows, max_cols);
    let Ok(command) = user_command else {
        let duration = start.elapsed();
        if *enable_output {
            print_sheet(1, 1, spreadsheet, max_rows, max_cols);
        }
        print!("[{:.1}] (invalid command) > ", duration.as_secs_f64());
        io::stdout().flush().unwrap();
        return;
    };
    let status;
    match command {
        Command::Range(cmd) => {
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
                (cell.row - 1, cell.col - 1),
                Some((operand_1.row - 1, operand_1.col - 1)),
                Some((operand_2.row - 1, operand_2.col - 1)),
                None,
                None,
                t,
            ) {
                Status::Ok => true,
                Status::Err => false,
            };

            if *enable_output {
                print_sheet(1, 1, spreadsheet, max_rows, max_cols);
            }
        }
        Command::Arithmetic(cmd) => {
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
                (cell.row - 1, cell.col - 1),
                cell_1,
                cell_2,
                const_1,
                const_2,
                t,
            ) {
                Status::Ok => true,
                Status::Err => false,
            };

            if *enable_output {
                print_sheet(1, 1, spreadsheet, max_rows, max_cols);
            }
        }
        Command::UserInteraction(cmd) => {
            let ui_command = cmd.command.clone();
            match ui_command.as_str() {
                "enable_output" => {
                    *enable_output = true;
                    print_sheet(*row, *col, spreadsheet, max_rows, max_cols);
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
                        print_sheet(*row, *col, spreadsheet, max_rows, max_cols);
                    }
                }
                "a" => {
                    if *enable_output {
                        if *col > 10 {
                            *col -= 10;
                        } else {
                            *col = 1;
                        };
                        print_sheet(*row, *col, spreadsheet, max_rows, max_cols);
                    }
                }
                "s" => {
                    if *enable_output {
                        *row = ((*row) + 10).min(max_rows - 9);
                        print_sheet(*row, *col, spreadsheet, max_rows, max_cols);
                    }
                }
                "d" => {
                    if *enable_output {
                        *col = ((*col) + 10).min(max_cols - 9);
                        print_sheet(*row, *col, spreadsheet, max_rows, max_cols);
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
                            print_sheet(*row, *col, spreadsheet, max_rows, max_cols);
                        }
                    };
                }
                _ => (),
            }
            status = true;
        }
        Command::Sleep(cmd) => {
            let Value::Cell(target_cell) = cmd.target_cell else {
                panic!();
            };

            let (cell_1, const_1) = match cmd.value {
                Value::Cell(cell) => (Some((cell.row - 1, cell.col - 1)), None),
                Value::Constant(constant) => (None, Some(constant)),
            };

            let t = Type::Slp;

            status = match spreadsheet.set_cell_equation(
                (target_cell.row - 1, target_cell.col - 1),
                cell_1,
                None,
                const_1,
                None,
                t,
            ) {
                Status::Ok => true,
                Status::Err => false,
            };

            if *enable_output {
                print_sheet(1, 1, spreadsheet, max_rows, max_cols);
            }
        }
        Command::Vcs(cmd) => {
            status = true;
            let command = cmd.command.clone();
            let command = command.as_str();
            match command {
                "list" => {
                    vcs.list();
                }
                "commit" => {
                    if let Some(argument) = cmd.argument {
                        vcs.commit(&argument, spreadsheet);
                    }
                }
                "checkout" => {
                    if let Some(argument) = cmd.argument {
                        if let Ok(commit_id) = argument[0..].parse::<usize>() {
                            let new_spreadsheet = vcs.checkout(commit_id);
                            *spreadsheet = new_spreadsheet;
                        }
                        if *enable_output {
                            print_sheet(1, 1, spreadsheet, max_rows, max_cols);
                        }
                    }
                }
                _ => (),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spreadsheet::SpreadSheet;
    use crate::vcs::vcs_engine::VersionControl;

    #[test]
    fn test_invalid_command() {
        let mut spreadsheet = SpreadSheet::new(10, 10);
        let mut vcs = VersionControl::new("./vcs_test".to_string(), &10, &10);
        let mut row = 1;
        let mut col = 1;
        let mut enable_output = false;
        let mut quit = false;

        // Test an invalid command
        process_command(
            "",
            &mut spreadsheet,
            &mut vcs,
            &mut row,
            &mut col,
            &mut enable_output,
            &mut quit,
        );

        // Ensure no changes occurred
        assert_eq!(row, 1);
        assert_eq!(col, 1);
        assert!(!quit);
    }

    #[test]
    fn test_arithmetic_command() {
        let mut spreadsheet = SpreadSheet::new(10, 10);
        let mut vcs = VersionControl::new("./vcs_test".to_string(), &10, &10);
        let mut row = 1;
        let mut col = 1;
        let mut enable_output = false;
        let mut quit = false;

        // Set initial values
        spreadsheet._set_cell_value(1, 0, 10);
        spreadsheet._set_cell_value(2, 0, 20);

        // Test an arithmetic command
        process_command(
            "A1 = A2 + A3",
            &mut spreadsheet,
            &mut vcs,
            &mut row,
            &mut col,
            &mut enable_output,
            &mut quit,
        );

        // Ensure the result is correct
        assert_eq!(spreadsheet.get_cell_value(0, 0), Some(30));
    }

    #[test]
    fn test_ui_commands_enable_disable() {
        let mut spreadsheet = SpreadSheet::new(10, 10);
        let mut vcs = VersionControl::new("./vcs_test".to_string(), &10, &10);
        let mut row = 1;
        let mut col = 1;
        let mut enable_output = false;
        let mut quit = false;

        // Enable output
        process_command(
            "enable_output",
            &mut spreadsheet,
            &mut vcs,
            &mut row,
            &mut col,
            &mut enable_output,
            &mut quit,
        );
        assert!(enable_output);

        // Disable output
        process_command(
            "disable_output",
            &mut spreadsheet,
            &mut vcs,
            &mut row,
            &mut col,
            &mut enable_output,
            &mut quit,
        );
        assert!(!enable_output);
    }

    #[test]
    fn test_ui_movement_commands() {
        let mut spreadsheet = SpreadSheet::new(100, 100);
        let mut vcs = VersionControl::new("./vcs_test".to_string(), &10, &10);
        let mut row = 5;
        let mut col = 5;
        let mut enable_output = true;
        let mut quit = false;

        // Move up
        process_command(
            "w",
            &mut spreadsheet,
            &mut vcs,
            &mut row,
            &mut col,
            &mut enable_output,
            &mut quit,
        );
        assert_eq!(row, 1);

        // Move left
        process_command(
            "a",
            &mut spreadsheet,
            &mut vcs,
            &mut row,
            &mut col,
            &mut enable_output,
            &mut quit,
        );
        assert_eq!(col, 1);

        // Move down
        process_command(
            "s",
            &mut spreadsheet,
            &mut vcs,
            &mut row,
            &mut col,
            &mut enable_output,
            &mut quit,
        );
        assert_eq!(row, 11);

        // Move right
        process_command(
            "d",
            &mut spreadsheet,
            &mut vcs,
            &mut row,
            &mut col,
            &mut enable_output,
            &mut quit,
        );
        assert_eq!(col, 11);
    }

    #[test]
    fn test_ui_quit_command() {
        let mut spreadsheet = SpreadSheet::new(10, 10);
        let mut vcs = VersionControl::new("./vcs_test".to_string(), &10, &10);
        let mut row = 1;
        let mut col = 1;
        let mut enable_output = false;
        let mut quit = false;

        // Quit command
        process_command(
            "q",
            &mut spreadsheet,
            &mut vcs,
            &mut row,
            &mut col,
            &mut enable_output,
            &mut quit,
        );
        assert!(quit);
    }

    #[test]
    fn test_vcs_commands() {
        let mut spreadsheet = SpreadSheet::new(10, 10);
        let mut vcs = VersionControl::new("./vcs_test".to_string(), &10, &10);
        let mut row = 1;
        let mut col = 1;
        let mut enable_output = false;
        let mut quit = false;

        // Commit command
        process_command(
            "gitsap commit Initial_commit",
            &mut spreadsheet,
            &mut vcs,
            &mut row,
            &mut col,
            &mut enable_output,
            &mut quit,
        );

        // List command
        process_command(
            "gitsap list",
            &mut spreadsheet,
            &mut vcs,
            &mut row,
            &mut col,
            &mut enable_output,
            &mut quit,
        );

        // Checkout command
        // process_command("gitsap checkout 1", &mut spreadsheet, &mut vcs, &mut row, &mut col, &mut enable_output, &mut quit);

        // Ensure the VCS state is correct
        assert_eq!(vcs.get_m_n(), (10, 10));
    }
}
