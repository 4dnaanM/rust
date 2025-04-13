use crate::parser::cell::Value;
#[derive(PartialEq, Debug, Clone)]
pub struct RangeCommand {
    pub target_cell: Value,
    pub function: String,
    pub operand_1: Value,
    pub operand_2: Value,
}

impl RangeCommand {
    pub fn is_valid_range_command(&self) -> bool {
        if let Value::Constant(_) = self.target_cell {
            return false;
        }

        if let Value::Cell(cell) = self.target_cell {
            if !cell.is_valid_cell() {
                return false;
            }
        }

        if let Value::Constant(_) = self.operand_1 {
            return false;
        }

        if let Value::Constant(_) = self.operand_2 {
            return false;
        }

        match (self.operand_1, self.operand_2) {
            (Value::Cell(cell_1), Value::Cell(cell_2)) => {
                cell_1.is_valid_cell() && cell_2.is_valid_cell() && cell_1.compare_cells(&cell_2)
            }
            _ => false,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct ArithmeticCommand {
    pub target_cell: Value,
    pub operand_1: Value,
    pub operator: Option<String>,
    pub operand_2: Option<Value>,
}

impl ArithmeticCommand {
    pub fn is_valid_arithmetic_command(&self) -> bool {
        if let Value::Constant(_) = self.target_cell {
            return false;
        }

        if let Value::Cell(cell) = self.target_cell {
            if !cell.is_valid_cell() {
                return false;
            }
        }

        if let Value::Cell(cell) = self.operand_1 {
            if !cell.is_valid_cell() {
                return false;
            }
        }

        match (&self.operator, self.operand_2) {
            (Some(_), Some(operand_2)) => match operand_2{
                Value::Cell(cell) => {
                    if !cell.is_valid_cell() {
                        return false;
                    }
                },
                Value::Constant(_) => ()
            },
            (None, Some(_)) => {return false;},
            (Some(_), None) => {return false;},
            (None, None) => ()
        };

        return true;
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct UserInteractionCommand {
    pub command: String,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Command {
    RangeCommand(RangeCommand),
    ArithmeticCommand(ArithmeticCommand),
    UserInteractionCommand(UserInteractionCommand),
}

impl Command {
    pub fn print_command(&self) {
        match self {
            Command::RangeCommand(cmd) => {
                println!("RangeCommand:");
                println!("  target_cell: {:?}", cmd.target_cell);
                println!("  function: {}", cmd.function);
                println!("  operand_1: {:?}", cmd.operand_1);
                println!("  operand_2: {:?}", cmd.operand_2);
            }
            Command::ArithmeticCommand(cmd) => {
                println!("ArithmeticCommand:");
                println!("  target_cell: {:?}", cmd.target_cell);
                println!("  operand_1: {:?}", cmd.operand_1);
                println!("  operator: {:?}", cmd.operator);
                println!("  operand_2: {:?}", cmd.operand_2);
            }
            Command::UserInteractionCommand(cmd) => {
                println!("UserInteractionCommand:");
                println!("  command: {}", cmd.command);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::cell::{Cell, Value};

    #[test]
    fn test_is_valid_range_command_returns_true() {
        let cmd = RangeCommand {
            target_cell: Value::Cell(Cell { row: 1, col: 1 }),
            function: "SUM".to_string(),
            operand_1: Value::Cell(Cell { row: 1, col: 1 }),
            operand_2: Value::Cell(Cell { row: 3, col: 3 }),
        };
        assert!(cmd.is_valid_range_command());
    }

    #[test]
    fn test_is_valid_range_command_returns_false_constant_values() {
        let cmds = vec![
            RangeCommand {
                target_cell: Value::Constant(10),
                function: "SUM".to_string(),
                operand_1: Value::Cell(Cell { row: 1, col: 1 }),
                operand_2: Value::Cell(Cell { row: 3, col: 3 }),
            },
            RangeCommand {
                target_cell: Value::Cell(Cell { row: 1, col: 1 }),
                function: "SUM".to_string(),
                operand_1: Value::Constant(10),
                operand_2: Value::Cell(Cell { row: 3, col: 3 }),
            },
            RangeCommand {
                target_cell: Value::Cell(Cell { row: 1, col: 1 }),
                function: "SUM".to_string(),
                operand_1: Value::Cell(Cell { row: 1, col: 1 }),
                operand_2: Value::Constant(10),
            },
        ];
        for cmd in cmds {
            assert!(!cmd.is_valid_range_command());
        }
    }

    #[test]
    fn test_is_valid_range_command_returns_false_invalid_operand_range() {
        let cmd = RangeCommand {
            target_cell: Value::Cell(Cell { row: 1, col: 1 }),
            function: "SUM".to_string(),
            operand_1: Value::Cell(Cell { row: 5, col: 5 }),
            operand_2: Value::Cell(Cell { row: 2, col: 2 }),
        };
        assert!(!cmd.is_valid_range_command());
    }

    #[test]
    fn test_is_valid_arithmetic_command_returns_true_one_operand() {
        let cmd = ArithmeticCommand {
            target_cell: Value::Cell(Cell { row: 1, col: 1 }),
            operand_1: Value::Cell(Cell { row: 2, col: 2 }),
            operator: None,
            operand_2: None,
        };
        assert!(cmd.is_valid_arithmetic_command());
    }

    #[test]
    fn test_is_valid_arithmetic_command_returns_true_two_operands() {
        let cmd = ArithmeticCommand {
            target_cell: Value::Cell(Cell { row: 1, col: 1 }),
            operand_1: Value::Cell(Cell { row: 2, col: 2 }),
            operator: Some("+".to_string()),
            operand_2: Some(Value::Constant(10)),
        };
        assert!(cmd.is_valid_arithmetic_command());
    }
    
    #[test]
    fn test_is_valid_arithmetic_command_returns_false_target_constant() {
        let cmd = ArithmeticCommand {
            target_cell: Value::Constant(1),
            operand_1: Value::Constant(2),
            operator: Some("*".to_string()),
            operand_2: Some(Value::Constant(3)),
        };
        assert!(!cmd.is_valid_arithmetic_command());
    }

    #[test]
    fn test_is_valid_arithmetic_command_returns_false_missing_operand_2() {
        let cmd = ArithmeticCommand {
            target_cell: Value::Cell(Cell { row: 1, col: 1 }),
            operand_1: Value::Constant(5),
            operator: Some("-".to_string()),
            operand_2: None,
        };
        assert!(!cmd.is_valid_arithmetic_command());
    }

    #[test]
    fn test_is_valid_arithmetic_command_returns_false_missing_operator() {
        let cmd = ArithmeticCommand {
            target_cell: Value::Cell(Cell { row: 1, col: 1 }),
            operand_1: Value::Constant(5),
            operator: None,
            operand_2: Some(Value::Constant(10)),
        };
        assert!(!cmd.is_valid_arithmetic_command());
    }

    #[test]
    fn test_invalid_arithmetic_command_invalid_cell() {
        let cmd = ArithmeticCommand {
            target_cell: Value::Cell(Cell { row: 0, col: 0 }),
            operand_1: Value::Cell(Cell { row: 1, col: 1 }),
            operator: Some("/".to_string()),
            operand_2: Some(Value::Constant(5)),
        };
        assert!(!cmd.is_valid_arithmetic_command());
    }
}
