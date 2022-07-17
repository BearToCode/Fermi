use crate::core::functions::operators::operator_data_from_char;

pub fn are_brackets_valid(expression: &String) -> Result<(), String> {
    let mut current_depth: i16 = 0;
    for (i, char) in expression.chars().enumerate() {
        if char == '(' {
            current_depth += 1;
        } else if char == ')' {
            current_depth -= 1;
            if current_depth < 0 {
                let message = format!("Invalid bracket at position: {}", i);
                return Err(message);
            }
        }
    }

    if current_depth != 0 {
        return Err(String::from("Invalid brackets!"));
    } else {
        Ok(())
    }
}

pub fn are_operators_valid(expression: &String) -> Result<(), String> {
    let mut previous_is_operator = false;

    for (index, char) in expression.chars().enumerate() {
        if !is_possible_operator(&char) {
            previous_is_operator = false;
            continue;
        }
        match operator_data_from_char(&char) {
            Some(_) => {
                // Avoid adjacent operators
                if previous_is_operator {
                    return Err(String::from(format!(
                        "Invalid adjacent operator '{}' at position: {} !",
                        char, index
                    )));
                }
                previous_is_operator = true;
            }
            None => {
                return Err(String::from(format!(
                    "Invalid operator '{}' at position: {} !",
                    char, index
                )))
            }
        }
    }
    Ok(())
}

// Operators are non digit, non alphabetic characters and non brackets
fn is_possible_operator(char: &char) -> bool {
    !char.is_alphanumeric() && *char != '(' && *char != ')' && *char != ',' && *char != '.'
}
