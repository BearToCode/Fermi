mod checks;
mod operators;
mod utils;

use super::functions::Expression;

pub fn parse_function<'a>(s: &'a str) -> Result<Expression, String> {
    let mut string = String::from(s);

    // Check brackets
    match checks::are_brackets_valid(&string) {
        Ok(()) => (),
        Err(error) => return Err(String::from(error)),
    }

    // Remove spaces
    string = string.replace(" ", "");

    println!("{}", string);

    // Convert operators
    match operators::operators_to_functions(&mut string) {
        Ok(()) => (),
        Err(err) => return Err(String::from(err)),
    };

    Err(String::from(string))
}