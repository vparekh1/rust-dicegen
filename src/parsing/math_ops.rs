extern crate nom;

use nom::{
    character::complete::{anychar, digit1, one_of, space0},
    combinator::map_res,
    multi::many0,
    sequence::tuple,
    IResult,
};

use super::{number, Operation};

fn roll_type(input: &str) -> IResult<&str, Operation> {
    map_res(one_of("+-/*"), parse_operation)(input)
}

fn parse_operation(op_char: char) -> Result<Operation, &'static str> {
    match op_char {
        '+' => Ok(Operation::Add),
        '-' => Ok(Operation::Sub),
        '/' => Ok(Operation::Div),
        '*' => Ok(Operation::Mul),
        _ => Err("Invalid Operation type. Expected one of +, -, /, or *"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operation_parses_as_expected() {
        assert_eq!(parse_operation('+'), Ok(Operation::Add));
    }

    #[test]
    fn op_parser_parses_operation() {
        assert_eq!(roll_type("/abcd"), Ok(("abcd", Operation::Div)))
    }
}
