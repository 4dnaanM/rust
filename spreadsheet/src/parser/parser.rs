use regex::Regex;
use crate::parser::cell::convert_string_to_cell;
use crate::parser::cell::Value;
use crate::parser::command::ArithmeticCommand;
use crate::parser::command::RangeCommand;
use crate::parser::command::UserInteractionCommand;
use crate::parser::command::Command;
use crate::parser::error::Error;

/// Parses user input
pub fn parse_cmd(user_command: &str) -> Result<Command, Error> {
    
    let ui_command: String = String::from(r"^(?P<UI_COMMAND>w|d|a|s|q|(enable_output)|(disable_output))$");
    let cell: String = String::from(r"[A-Z]+[0-9]+");
    let constant: String = String::from(r"(-)?[0-9]+");
    let operator: String = String::from(r"(\+|\-|\*|\/)");
    let function: String = String::from(r"(MAX|MIN|AVG|STDEV|SUM)");
    let value: String = format!("({}|{})", cell, constant);
    let range_cmd: String = format!("^(\\s*(?P<TARGET_CELL_RANGE>{})\\s*=\\s*(?P<FUNCTION>{})\\(\\s*(?P<OPERAND_1_RANGE>{})\\s*:\\s*(?P<OPERAND_2_RANGE>{})\\s*\\)\\s*)$", cell, function, cell, cell);
    let arithmetic_cmd: String = format!("^(\\s*(?P<TARGET_CELL_ARTH>{})\\s*=\\s*(?P<OPERAND_1_ARTH>{})\\s*((?P<OPERATOR>{})\\s*(?P<OPERAND_2_ARTH>{})\\s*)?)$", cell, value, operator, value);
    
    // A valid command is either a UI command, range command or arithmetic command
    let command: String = format!("{}|{}|{}", ui_command, range_cmd, arithmetic_cmd);
    let regex = Regex::new(&command).map_err(|e| Error::RegexError(e.to_string()))?;

    let captured_groups = regex.captures(user_command);

    // If none of the groups are captured, then the user input in invalid
    let Some(captures) = captured_groups else {
        return Err(Error::InvalidInput(String::from("Invalid user input")));
    };

    // First, check for UI command
    if let Some(command) = captures.name("UI_COMMAND"){
        let user_interaction = UserInteractionCommand {command: command.as_str().to_string()};
        return Ok(Command::UserInteractionCommand(user_interaction));
    }

    // Second, check for range command
    let range_cmd_groups = ["TARGET_CELL_RANGE", "FUNCTION", "OPERAND_1_RANGE", "OPERAND_2_RANGE"];
    let is_range_command = range_cmd_groups.iter().all(|&g| captures.name(g).is_some());
    if is_range_command {
        let target_cell_str = captures.name("TARGET_CELL_RANGE").ok_or_else(|| Error::InvalidInput("Invalid user input".to_string()))?.as_str();
        let target_cell = match convert_string_to_cell(target_cell_str) {
            Some(target_cell) => target_cell,
            None => {return Err(Error::InvalidInput(String::from("Invalid user input")))},
        };

        let operand_1_str = captures.name("OPERAND_1_RANGE").ok_or_else(|| Error::InvalidInput("Invalid user input".to_string()))?.as_str();
        let operand_1 = match convert_string_to_cell(operand_1_str) {
            Some(operand_1) => operand_1,
            None => {return Err(Error::InvalidInput(String::from("Invalid user input")))},
        };

        let operand_2_str = captures.name("OPERAND_2_RANGE").ok_or_else(|| Error::InvalidInput("Invalid user input".to_string()))?.as_str();
        let operand_2 = match convert_string_to_cell(operand_2_str) {
            Some(operand_2) => operand_2,
            None => {return Err(Error::InvalidInput(String::from("Invalid user input")))},
        };

        let cmd = RangeCommand {
            target_cell: Value::Cell(target_cell),
            function: captures.name("FUNCTION").ok_or_else(|| Error::InvalidInput("Invalid user input".to_string()))?.as_str().to_string(),
            operand_1: Value::Cell(operand_1),
            operand_2: Value::Cell(operand_2)
        };
        if !cmd.is_valid_range_command() {
            return Err(Error::InvalidInput("Invalid user input".to_string()));
        }
        return Ok(Command::RangeCommand(cmd));
    }

    // Third, check for arithmetic command
    let arithmetic_cmd_groups_required = ["TARGET_CELL_ARTH", "OPERAND_1_ARTH"];
    let arithmetic_cmd_groups_optional = ["OPERATOR", "OPERAND_2_ARTH"];
    let required_all_present = arithmetic_cmd_groups_required.iter().all(|&g| captures.name(g).is_some());
    let optional_all_present = arithmetic_cmd_groups_optional.iter().all(|&g| captures.name(g).is_some());
    if required_all_present {
        let target_cell_str = captures.name("TARGET_CELL_ARTH").ok_or_else(|| Error::InvalidInput("Invalid user input".to_string()))?.as_str();
        let target_cell = match convert_string_to_cell(target_cell_str) {
            Some(target_cell) => target_cell,
            None => {return Err(Error::InvalidInput(String::from("Invalid user input")))},
        };

        let operand_1_str = captures.name("OPERAND_1_ARTH").ok_or_else(|| Error::InvalidInput("Invalid user input".to_string()))?.as_str();
        let operand_1 = match operand_1_str.parse::<i32>() {
            Ok(constant) => Value::Constant(constant),
            Err(_) => match convert_string_to_cell(operand_1_str) {
                Some(operand_1) => Value::Cell(operand_1),
                None => {return Err(Error::InvalidInput(String::from("Invalid user input")))},
            }
        }; 
        if optional_all_present {
            let operand_2_str = captures.name("OPERAND_2_ARTH").ok_or_else(|| Error::InvalidInput("Invalid user input".to_string()))?.as_str();
            let operand_2 = match operand_2_str.parse::<i32>() {
                Ok(constant) => Value::Constant(constant),
                Err(_) => match convert_string_to_cell(operand_2_str) {
                    Some(operand_2) => Value::Cell(operand_2),
                    None => {return Err(Error::InvalidInput(String::from("Invalid user input")))},
                }
            }; 
            let cmd = ArithmeticCommand {
                target_cell: Value::Cell(target_cell),
                operand_1: operand_1,
                operator: Some(captures.name("OPERATOR").ok_or_else(|| Error::InvalidInput("Invalid user input".to_string()))?.as_str().to_string()),
                operand_2: Some(operand_2),
            };
            if !cmd.is_valid_arithmetic_command(){
                return Err(Error::InvalidInput("Invalid user input".to_string()));
            }
            return Ok(Command::ArithmeticCommand(cmd));
        } else {
            let cmd = ArithmeticCommand {
                target_cell: Value::Cell(target_cell),
                operand_1: operand_1,
                operator: None,
                operand_2: None,
            };
            if !cmd.is_valid_arithmetic_command(){
                return Err(Error::InvalidInput("Invalid user input".to_string()));
            }
            return Ok(Command::ArithmeticCommand(cmd));            
        }
    }
    
    // The command does not match the regex. Therefore, invalid command
    return Err(Error::InvalidInput("Invalid user input".to_string()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ui_command() {
        let input = "w";
        let result = parse_cmd(input);
        assert!(matches!(result, Ok(Command::UserInteractionCommand(_))));
    }

    #[test]
    fn test_range_command_valid() {
        let input = "A1 = SUM(B1:C1)";
        let result = parse_cmd(input);
        assert!(matches!(result, Ok(Command::RangeCommand(_))));
    }

    #[test]
    fn test_range_command_invalid_cell() {
        let input = "Z0 = MAX(XYZ:123)";
        let result = parse_cmd(input);
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }

    #[test]
    fn test_arithmetic_command_with_operator() {
        let input = "A1 = B1 + 3";
        let result = parse_cmd(input);
        assert!(matches!(result, Ok(Command::ArithmeticCommand(_))));
    }

    #[test]
    fn test_arithmetic_command_without_operator() {
        let input = "A1 = 42";
        let result = parse_cmd(input);
        assert!(matches!(result, Ok(Command::ArithmeticCommand(_))));
    }

    #[test]
    fn test_invalid_command_format() {
        let input = "run macro";
        let result = parse_cmd(input);
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }

    #[test]
    fn test_invalid_operator_expression() {
        let input = "A1 = B1 +";
        let result = parse_cmd(input);
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }
}
