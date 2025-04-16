use crate::equation::Equation;
use crate::parser;
use crate::parser::cell::Value;
use crate::parser::command::Command;
use crate::parser::error::Error;
use crate::parser::print_output::print_sheet;
use crate::spreadsheet::SpreadSheet;
use std::time::Instant;
use std::io::{self, Write};
use crate::utils::Type;
use crate::utils::Coordinate;
use crate::operand::SharedOperand;
use crate::operand::Operand;

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
            let Value::Cell(cell) = cmd.target_cell else {
                panic!();  
            };
            let Value::Cell(operand_1) = cmd.operand_1 else {
                panic!();
            };
            let Value::Cell(operand_2) = cmd.operand_2 else {
                panic!();
            };
            let t = Some(Type::from_str(cmd.function.as_str()));
            let operands = Some(vec![spreadsheet.cells[operand_1.row-1][operand_1.col-1].clone(), spreadsheet.cells[operand_2.row-1][operand_2.col-1].clone() ]);
            let equation = Equation::new(Coordinate(cell.row-1, cell.col-1), t, operands);
            spreadsheet.set_cell_equation(cell.row-1, cell.col-1, equation);
            
            if *enable_output {
                *row = cell.row;
                *col = cell.col;    
                print_sheet(*row , *col, spreadsheet, *max_rows, *max_cols);
            }
        },
        Command::ArithmeticCommand(cmd) => {
            let Value::Cell(cell) = cmd.target_cell else {
                panic!();  
            };

            let operand_1 = match cmd.operand_1 {
                Value::Cell(cell_1) => {
                    spreadsheet.cells[cell_1.row-1][cell_1.col-1].clone()
                },
                Value::Constant(constant) => {
                    SharedOperand::new(Operand::new(Some((0, 1)), Some(constant)))
                }
            };

            let operand_2 = match cmd.operand_2 {
                Some(operand_2) => match operand_2 {
                    Value::Cell(cell_2) => {
                        spreadsheet.cells[cell_2.row-1][cell_2.col-1].clone()
                    },
                    Value::Constant(constant) => {
                        SharedOperand::new(Operand::new(Some((0, 1)), Some(constant)))
                    }
                },
                None => SharedOperand::new(Operand::new(Some((0, 1)), Some(0)))
            };

            let t = match cmd.operator {
                Some(op) => Some(Type::from_str(op.as_str())),
                None => Some(Type::from_str("+"))
            };

            let operands = Some(vec![operand_1, operand_2]);
            let equation = Equation::new(Coordinate(cell.row-1, cell.col-1), t, operands);
            spreadsheet.set_cell_equation(cell.row-1, cell.col-1, equation);

            if *enable_output {
                *row = cell.row;
                *col = cell.col;    
                print_sheet(*row , *col, spreadsheet, *max_rows, *max_cols);
            }

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
                        print_sheet(*row , *col, spreadsheet, *max_rows, *max_cols);
                    }
                },
                "a" => {
                    if *enable_output {
                        if *col > 10 {
                            *col = (*col) - 10;
                        } else {
                            *col = 1;
                        };
                        print_sheet(*row , *col, spreadsheet, *max_rows, *max_cols);
                    }
                },
                "s" => {
                    if *enable_output {
                        *row = ((*row) + 10).min(*max_rows);
                        print_sheet(*row , *col, spreadsheet, *max_rows, *max_cols);
                    }
                },
                "d" => {
                    if *enable_output {
                        *col = ((*col) + 10).min(*max_cols);
                        print_sheet(*row , *col, spreadsheet, *max_rows, *max_cols);
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
