use crate::parser::cell::Value;
use crate::parser::cell::convert_string_to_cell;
use crate::parser::command::ArithmeticCommand;
use crate::parser::command::Command;
use crate::parser::command::RangeCommand;
use crate::parser::command::UserInteractionCommand;
use crate::parser::error::Error;
use regex::Regex;

use super::command::SleepCommand;
use super::command::VCSCommand;

/// Parses user input
pub fn parse_cmd(user_command: &str, max_rows: usize, max_cols: usize) -> Result<Command, Error> {
    let cell: String = String::from(r"[A-Z]+[0-9]+");
    let constant: String = String::from(r"(-)?[0-9]+");
    let operator: String = String::from(r"(\+|\-|\*|\/)");
    let function: String = String::from(r"(MAX|MIN|AVG|STDEV|SUM)");
    let value: String = format!("({}|{})", cell, constant);

    let vcs_command: String = String::from(
        "^(gitsap\\s(?P<VCS_COMMAND>commit|list|checkout)(\\s(?P<VCS_INFO>[A-Za-z0-9]+))?\\s*)$",
    );

    let ui_command: String = format!(
        "^((?P<UI_COMMAND>w|d|a|s|q|(\\s*enable_output\\s*)|(\\s*disable_output\\s*))|(\\s*scroll_to (?P<SCROLL_TO_CELL>{})\\s*))$",
        cell
    );
    let sleep_command: String = format!(
        "^((?P<SLEEP_TARGET_CELL>{})\\s*=\\s*SLEEP\\(\\s*(?P<SLEEP_VALUE>{})\\s*\\)\\s*)$",
        cell, value
    );
    let range_cmd: String = format!(
        "^(\\s*(?P<TARGET_CELL_RANGE>{})\\s*=\\s*(?P<FUNCTION>{})\\(\\s*(?P<OPERAND_1_RANGE>{})\\s*:\\s*(?P<OPERAND_2_RANGE>{})\\s*\\)\\s*)$",
        cell, function, cell, cell
    );
    let arithmetic_cmd: String = format!(
        "^(\\s*(?P<TARGET_CELL_ARTH>{})\\s*=\\s*(?P<OPERAND_1_ARTH>{})\\s*((?P<OPERATOR>{})\\s*(?P<OPERAND_2_ARTH>{})\\s*)?)$",
        cell, value, operator, value
    );

    // A valid command is either a UI command, range command or arithmetic command
    let command: String = format!(
        "{}|{}|{}|{}|{}",
        vcs_command, ui_command, sleep_command, range_cmd, arithmetic_cmd
    );
    let regex = Regex::new(&command).map_err(|_| Error::RegexError)?;

    let captured_groups = regex.captures(user_command);

    // If none of the groups are captured, then the user input in invalid
    let Some(captures) = captured_groups else {
        return Err(Error::InvalidInput);
    };

    // Check for VCS Command
    if let Some(command) = captures.name("VCS_COMMAND") {
        let vcs_command;
        if let Some(info) = captures.name("VCS_INFO") {
            vcs_command = VCSCommand {
                command: command.as_str().to_string(),
                argument: Some(info.as_str().to_string()),
            };
        } else {
            vcs_command = VCSCommand {
                command: command.as_str().to_string(),
                argument: None,
            };
        }
        if !vcs_command.is_valid_vcs_command() {
            return Err(Error::InvalidInput);
        }
        return Ok(Command::Vcs(vcs_command));
    }

    // First, check for UI command
    if let Some(command) = captures.name("UI_COMMAND") {
        let user_interaction = UserInteractionCommand {
            command: command.as_str().to_string(),
            scroll_to_cell: None,
        };
        return Ok(Command::UserInteraction(user_interaction));
    }

    if captures.name("SCROLL_TO_CELL").is_some() {
        let scroll_to_cell_str = captures
            .name("SCROLL_TO_CELL")
            .ok_or(Error::InvalidInput)?
            .as_str();
        let scroll_to_cell = match convert_string_to_cell(scroll_to_cell_str) {
            Some(scroll_to_cell) => scroll_to_cell,
            None => return Err(Error::InvalidInput),
        };
        let user_interaction = UserInteractionCommand {
            command: String::from("scroll_to"),
            scroll_to_cell: Some(scroll_to_cell),
        };
        if !user_interaction.is_valid_ui_command(max_rows, max_cols) {
            return Err(Error::InvalidInput);
        }
        return Ok(Command::UserInteraction(user_interaction));
    }

    // Second, check for sleep command
    let sleep_command_groups = ["SLEEP_TARGET_CELL", "SLEEP_VALUE"];
    let is_sleep_command = sleep_command_groups
        .iter()
        .all(|&g| captures.name(g).is_some());
    if is_sleep_command {
        let sleep_target_cell_str = captures
            .name("SLEEP_TARGET_CELL")
            .ok_or(Error::InvalidInput)?
            .as_str();
        let sleep_target_cell = match convert_string_to_cell(sleep_target_cell_str) {
            Some(target_cell) => target_cell,
            None => return Err(Error::InvalidInput),
        };

        let sleep_value_str = captures
            .name("SLEEP_VALUE")
            .ok_or(Error::InvalidInput)?
            .as_str();
        let sleep_value = match sleep_value_str.parse::<i32>() {
            Ok(constant) => Value::Constant(constant),
            Err(_) => match convert_string_to_cell(sleep_value_str) {
                Some(sleep_value) => Value::Cell(sleep_value),
                None => return Err(Error::InvalidInput),
            },
        };

        let sleep_command = SleepCommand {
            target_cell: Value::Cell(sleep_target_cell),
            value: sleep_value,
        };
        if !sleep_command.is_valid_sleep_command(max_rows, max_cols) {
            return Err(Error::InvalidInput);
        }
        return Ok(Command::Sleep(sleep_command));
    }

    // Third, check for range command
    let range_cmd_groups = [
        "TARGET_CELL_RANGE",
        "FUNCTION",
        "OPERAND_1_RANGE",
        "OPERAND_2_RANGE",
    ];
    let is_range_command = range_cmd_groups.iter().all(|&g| captures.name(g).is_some());
    if is_range_command {
        let target_cell_str = captures
            .name("TARGET_CELL_RANGE")
            .ok_or(Error::InvalidInput)?
            .as_str();
        let target_cell = match convert_string_to_cell(target_cell_str) {
            Some(target_cell) => target_cell,
            None => return Err(Error::InvalidInput),
        };

        let operand_1_str = captures
            .name("OPERAND_1_RANGE")
            .ok_or(Error::InvalidInput)?
            .as_str();
        let operand_1 = match convert_string_to_cell(operand_1_str) {
            Some(operand_1) => operand_1,
            None => return Err(Error::InvalidInput),
        };

        let operand_2_str = captures
            .name("OPERAND_2_RANGE")
            .ok_or(Error::InvalidInput)?
            .as_str();
        let operand_2 = match convert_string_to_cell(operand_2_str) {
            Some(operand_2) => operand_2,
            None => return Err(Error::InvalidInput),
        };

        let cmd = RangeCommand {
            target_cell: Value::Cell(target_cell),
            function: captures
                .name("FUNCTION")
                .ok_or(Error::InvalidInput)?
                .as_str()
                .to_string(),
            operand_1: Value::Cell(operand_1),
            operand_2: Value::Cell(operand_2),
        };
        if !cmd.is_valid_range_command(max_rows, max_cols) {
            return Err(Error::InvalidInput);
        }
        return Ok(Command::Range(cmd));
    }

    // Fourth, check for arithmetic command
    let arithmetic_cmd_groups_required = ["TARGET_CELL_ARTH", "OPERAND_1_ARTH"];
    let arithmetic_cmd_groups_optional = ["OPERATOR", "OPERAND_2_ARTH"];
    let required_all_present = arithmetic_cmd_groups_required
        .iter()
        .all(|&g| captures.name(g).is_some());
    let optional_all_present = arithmetic_cmd_groups_optional
        .iter()
        .all(|&g| captures.name(g).is_some());
    if required_all_present {
        let target_cell_str = captures
            .name("TARGET_CELL_ARTH")
            .ok_or(Error::InvalidInput)?
            .as_str();
        let target_cell = match convert_string_to_cell(target_cell_str) {
            Some(target_cell) => target_cell,
            None => return Err(Error::InvalidInput),
        };

        let operand_1_str = captures
            .name("OPERAND_1_ARTH")
            .ok_or(Error::InvalidInput)?
            .as_str();
        let operand_1 = match operand_1_str.parse::<i32>() {
            Ok(constant) => Value::Constant(constant),
            Err(_) => match convert_string_to_cell(operand_1_str) {
                Some(operand_1) => Value::Cell(operand_1),
                None => return Err(Error::InvalidInput),
            },
        };
        if optional_all_present {
            let operand_2_str = captures
                .name("OPERAND_2_ARTH")
                .ok_or(Error::InvalidInput)?
                .as_str();
            let operand_2 = match operand_2_str.parse::<i32>() {
                Ok(constant) => Value::Constant(constant),
                Err(_) => match convert_string_to_cell(operand_2_str) {
                    Some(operand_2) => Value::Cell(operand_2),
                    None => return Err(Error::InvalidInput),
                },
            };
            let cmd = ArithmeticCommand {
                target_cell: Value::Cell(target_cell),
                operand_1,
                operator: Some(
                    captures
                        .name("OPERATOR")
                        .ok_or(Error::InvalidInput)?
                        .as_str()
                        .to_string(),
                ),
                operand_2: Some(operand_2),
            };
            if !cmd.is_valid_arithmetic_command(max_rows, max_cols) {
                return Err(Error::InvalidInput);
            }
            return Ok(Command::Arithmetic(cmd));
        } else {
            let cmd = ArithmeticCommand {
                target_cell: Value::Cell(target_cell),
                operand_1,
                operator: None,
                operand_2: None,
            };
            if !cmd.is_valid_arithmetic_command(max_rows, max_cols) {
                return Err(Error::InvalidInput);
            }
            return Ok(Command::Arithmetic(cmd));
        }
    }

    // The command does not match the regex. Therefore, invalid command
    Err(Error::InvalidInput)
}

#[cfg(test)]
mod tests {
    use super::*;
    const MAX_ROWS: usize = 999;
    const MAX_COLS: usize = 18278;

    #[test]
    fn test_ui_command() {
        let input = "w";
        let result = parse_cmd(input, MAX_ROWS, MAX_COLS);
        assert!(matches!(result, Ok(Command::UserInteraction(_))));
    }

    #[test]
    fn test_sleep_command_valid() {
        let input = "A1 = SLEEP(B1)";
        let result = parse_cmd(input, MAX_ROWS, MAX_COLS);
        assert!(matches!(result, Ok(Command::Sleep(_))));
    }

    // Test invalid sleep command (invalid cell in value)
    #[test]
    fn test_sleep_command_invalid_value() {
        let input = "A1 = SLEEP(ABC)";
        let result = parse_cmd(input, MAX_ROWS, MAX_COLS);
        assert!(matches!(result, Err(Error::InvalidInput)));
    }

    // Test invalid sleep command (invalid format)
    #[test]
    fn test_sleep_command_invalid_format() {
        let input = "A1 = SLEEP";
        let result = parse_cmd(input, MAX_ROWS, MAX_COLS);
        assert!(matches!(result, Err(Error::InvalidInput)));
    }

    // Test UI command with scroll
    #[test]
    fn test_ui_command_scroll_to_cell() {
        let input = "scroll_to A1";
        let result = parse_cmd(input, MAX_ROWS, MAX_COLS);
        assert!(matches!(result, Ok(Command::UserInteraction(_))));
    }

    // Test invalid UI command (unsupported command)
    #[test]
    fn test_invalid_ui_command() {
        let input = "unsupported_command";
        let result = parse_cmd(input, MAX_ROWS, MAX_COLS);
        assert!(matches!(result, Err(Error::InvalidInput)));
    }

    #[test]
    fn test_range_command_valid() {
        let input = "A1 = SUM(B1:C1)";
        let result = parse_cmd(input, MAX_ROWS, MAX_COLS);
        assert!(matches!(result, Ok(Command::Range(_))));
    }

    #[test]
    fn test_range_command_invalid_cell() {
        let input = "Z0 = MAX(XYZ:123)";
        let result = parse_cmd(input, MAX_ROWS, MAX_COLS);
        assert!(matches!(result, Err(Error::InvalidInput)));
    }

    #[test]
    fn test_arithmetic_command_with_operator() {
        let input = "A1 = B1 + 3";
        let result = parse_cmd(input, MAX_ROWS, MAX_COLS);
        assert!(matches!(result, Ok(Command::Arithmetic(_))));
    }

    #[test]
    fn test_arithmetic_command_with_two_cell_operands() {
        let input = "A1 = B1 + A1";
        let result = parse_cmd(input, MAX_ROWS, MAX_COLS);
        assert!(matches!(result, Ok(Command::Arithmetic(_))));
    }

    #[test]
    fn test_arithmetic_command_without_operator() {
        let input = "A1 = 42";
        let result = parse_cmd(input, MAX_ROWS, MAX_COLS);
        assert!(matches!(result, Ok(Command::Arithmetic(_))));
    }

    #[test]
    fn test_invalid_command_format() {
        let input = "run macro";
        let result = parse_cmd(input, MAX_ROWS, MAX_COLS);
        assert!(matches!(result, Err(Error::InvalidInput)));
    }

    #[test]
    fn test_invalid_operator_expression() {
        let input = "A1 = B1 +";
        let result = parse_cmd(input, MAX_ROWS, MAX_COLS);
        assert!(matches!(result, Err(Error::InvalidInput)));
    }
}
