use crate::parser::cell::Value;

use super::cell::Cell;

/// Represents function commands like A1=MAX(A2:A3)
#[derive(PartialEq, Debug, Clone)]
pub struct RangeCommand {
    pub target_cell: Value,
    pub function: String,
    pub operand_1: Value,
    pub operand_2: Value,
}

impl RangeCommand {
    /// Checks if the range command is valid
    pub fn is_valid_range_command(&self, max_rows: usize, max_cols: usize) -> bool {
        // Target must be a cell
        if let Value::Constant(_) = self.target_cell {
            return false;
        }

        // Cell must be within bounds
        if let Value::Cell(cell) = self.target_cell {
            if !cell.is_valid_cell(max_rows, max_cols) {
                return false;
            }
        }

        // Both operands must be cells
        if let Value::Constant(_) = self.operand_1 {
            return false;
        }
        if let Value::Constant(_) = self.operand_2 {
            return false;
        }

        // Both operands must be cells and must make a valid range
        match (self.operand_1, self.operand_2) {
            (Value::Cell(cell_1), Value::Cell(cell_2)) => {
                cell_1.is_valid_cell(max_rows, max_cols)
                    && cell_2.is_valid_cell(max_rows, max_cols)
                    && cell_1.compare_cells(&cell_2)
            }
            _ => false,
        }
    }
}

/// Represents an arithmetic command like A1=1 or A1=A1+2
#[derive(PartialEq, Debug, Clone)]
pub struct ArithmeticCommand {
    pub target_cell: Value,
    pub operand_1: Value,
    pub operator: Option<String>,
    pub operand_2: Option<Value>,
}

impl ArithmeticCommand {
    /// Checks if the arithmetic command is valid
    pub fn is_valid_arithmetic_command(&self, max_rows: usize, max_cols: usize) -> bool {
        // Target must be a valid cell
        if let Value::Constant(_) = self.target_cell {
            return false;
        }
        if let Value::Cell(cell) = self.target_cell {
            if !cell.is_valid_cell(max_rows, max_cols) {
                return false;
            }
        }

        // If any operand is a cell, it must be a valid cell
        if let Value::Cell(cell) = self.operand_1 {
            if !cell.is_valid_cell(max_rows, max_cols) {
                return false;
            }
        }

        match (&self.operator, self.operand_2) {
            (Some(_), Some(operand_2)) => match operand_2 {
                Value::Cell(cell) => {
                    if !cell.is_valid_cell(max_rows, max_cols) {
                        return false;
                    }
                }
                Value::Constant(_) => (),
            },
            (None, Some(_)) => {
                return false;
            }
            (Some(_), None) => {
                return false;
            }
            (None, None) => (),
        };

        true
    }
}

/// Represents user interaction command like q, w, a, s, d, enable_output, disable_output
#[derive(PartialEq, Debug, Clone)]
pub struct UserInteractionCommand {
    pub command: String,
    pub scroll_to_cell: Option<Cell>,
}

impl UserInteractionCommand {
    pub fn is_valid_ui_command(&self, max_rows: usize, max_cols: usize) -> bool {
        match self.scroll_to_cell {
            Some(cell) => cell.is_valid_cell(max_rows, max_cols),
            None => true,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct SleepCommand {
    pub target_cell: Value,
    pub value: Value,
}

impl SleepCommand {
    pub fn is_valid_sleep_command(&self, max_rows: usize, max_cols: usize) -> bool {
        match (self.target_cell, self.value) {
            (Value::Cell(target_cell), Value::Cell(cell)) => {
                target_cell.is_valid_cell(max_rows, max_cols)
                    && cell.is_valid_cell(max_rows, max_cols)
            }
            (Value::Cell(target_cell), Value::Constant(_)) => {
                target_cell.is_valid_cell(max_rows, max_cols)
            }
            _ => false,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct VCSCommand {
    pub command: String,
    pub argument: Option<String>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Command {
    RangeCommand(RangeCommand),
    ArithmeticCommand(ArithmeticCommand),
    UserInteractionCommand(UserInteractionCommand),
    SleepCommand(SleepCommand),
    VCSCommand(VCSCommand),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::cell::{Cell, Value};

    #[test]
    fn test_is_valid_range_command_returns_true() {
        let max_rows = 999;
        let max_cols = 18278;

        let cmd = RangeCommand {
            target_cell: Value::Cell(Cell { row: 1, col: 1 }),
            function: "SUM".to_string(),
            operand_1: Value::Cell(Cell { row: 1, col: 1 }),
            operand_2: Value::Cell(Cell { row: 3, col: 3 }),
        };
        assert!(cmd.is_valid_range_command(max_rows, max_cols));
    }

    #[test]
    fn test_is_valid_range_command_returns_false_constant_values() {
        let max_rows = 999;
        let max_cols = 18278;
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
            assert!(!cmd.is_valid_range_command(max_rows, max_cols));
        }
    }

    #[test]
    fn test_is_valid_range_command_returns_false_invalid_operand_range() {
        let max_rows = 999;
        let max_cols = 18278;
        let cmd = RangeCommand {
            target_cell: Value::Cell(Cell { row: 1, col: 1 }),
            function: "SUM".to_string(),
            operand_1: Value::Cell(Cell { row: 5, col: 5 }),
            operand_2: Value::Cell(Cell { row: 2, col: 2 }),
        };
        assert!(!cmd.is_valid_range_command(max_rows, max_cols));
    }

    #[test]
    fn test_is_valid_arithmetic_command_returns_true_one_operand() {
        let max_rows = 999;
        let max_cols = 18278;
        let cmd = ArithmeticCommand {
            target_cell: Value::Cell(Cell { row: 1, col: 1 }),
            operand_1: Value::Cell(Cell { row: 2, col: 2 }),
            operator: None,
            operand_2: None,
        };
        assert!(cmd.is_valid_arithmetic_command(max_rows, max_cols));
    }

    #[test]
    fn test_is_valid_arithmetic_command_returns_true_two_operands() {
        let max_rows = 999;
        let max_cols = 18278;
        let cmd = ArithmeticCommand {
            target_cell: Value::Cell(Cell { row: 1, col: 1 }),
            operand_1: Value::Cell(Cell { row: 2, col: 2 }),
            operator: Some("+".to_string()),
            operand_2: Some(Value::Constant(10)),
        };
        assert!(cmd.is_valid_arithmetic_command(max_rows, max_cols));
    }

    #[test]
    fn test_is_valid_arithmetic_command_returns_false_target_constant() {
        let max_rows = 999;
        let max_cols = 18278;
        let cmd = ArithmeticCommand {
            target_cell: Value::Constant(1),
            operand_1: Value::Constant(2),
            operator: Some("*".to_string()),
            operand_2: Some(Value::Constant(3)),
        };
        assert!(!cmd.is_valid_arithmetic_command(max_rows, max_cols));
    }

    #[test]
    fn test_is_valid_arithmetic_command_returns_false_missing_operand_2() {
        let max_rows = 999;
        let max_cols = 18278;
        let cmd = ArithmeticCommand {
            target_cell: Value::Cell(Cell { row: 1, col: 1 }),
            operand_1: Value::Constant(5),
            operator: Some("-".to_string()),
            operand_2: None,
        };
        assert!(!cmd.is_valid_arithmetic_command(max_rows, max_cols));
    }

    #[test]
    fn test_is_valid_arithmetic_command_returns_false_missing_operator() {
        let max_rows = 999;
        let max_cols = 18278;
        let cmd = ArithmeticCommand {
            target_cell: Value::Cell(Cell { row: 1, col: 1 }),
            operand_1: Value::Constant(5),
            operator: None,
            operand_2: Some(Value::Constant(10)),
        };
        assert!(!cmd.is_valid_arithmetic_command(max_rows, max_cols));
    }

    #[test]
    fn test_is_valid_arithmetic_command_returns_false_invalid_cell() {
        let max_rows = 999;
        let max_cols = 18278;
        let cmd = ArithmeticCommand {
            target_cell: Value::Cell(Cell { row: 0, col: 0 }),
            operand_1: Value::Cell(Cell { row: 1, col: 1 }),
            operator: Some("/".to_string()),
            operand_2: Some(Value::Constant(5)),
        };
        assert!(!cmd.is_valid_arithmetic_command(max_rows, max_cols));
    }

    #[test]
    fn test_is_valid_sleep_command_returns_true() {
        let max_rows = 999;
        let max_cols = 18278;
        let cmd = SleepCommand {
            target_cell: Value::Cell(Cell { row: 2, col: 2 }),
            value: Value::Cell(Cell { row: 20, col: 1 }),
        };
        assert!(cmd.is_valid_sleep_command(max_rows, max_cols));
    }

    #[test]
    fn test_is_valid_sleep_command_returns_false_target_is_constant() {
        let cmd = SleepCommand {
            target_cell: Value::Constant(5),
            value: Value::Constant(10),
        };
        assert!(!cmd.is_valid_sleep_command(10, 10));
    }

    #[test]
    fn test_is_valid_sleep_command_returns_false_target_cell_invalid() {
        let cmd = SleepCommand {
            target_cell: Value::Cell(Cell { row: 0, col: 0 }),
            value: Value::Constant(10),
        };
        assert!(!cmd.is_valid_sleep_command(10, 10));
    }

    #[test]
    fn test_is_valid_ui_command_returns_true_movement_commands() {
        let max_rows = 999;
        let max_cols = 18278;
        let cmd = UserInteractionCommand {
            command: "w".to_string(),
            scroll_to_cell: None,
        };
        assert!(cmd.is_valid_ui_command(max_rows, max_cols));
    }

    #[test]
    fn test_is_valid_ui_command_returns_true_scroll_command() {
        let max_rows = 999;
        let max_cols = 18278;
        let cmd = UserInteractionCommand {
            command: "scroll_to".to_string(),
            scroll_to_cell: Some(Cell { row: 10, col: 10 }),
        };
        assert!(cmd.is_valid_ui_command(max_rows, max_cols));
    }

    #[test]
    fn test_is_valid_ui_command_returns_false_invalid_cell() {
        let max_rows = 10;
        let max_cols = 10;
        let cmd = UserInteractionCommand {
            command: "scroll_to".to_string(),
            scroll_to_cell: Some(Cell { row: 10, col: 11 }),
        };
        assert!(!cmd.is_valid_ui_command(max_rows, max_cols));
    }
}
