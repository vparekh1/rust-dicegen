extern crate nom;
extern crate simple_error;

use nom::{
    character::complete::{anychar, digit1, one_of, space0},
    combinator::map_res,
    multi::many0,
    sequence::tuple,
    IResult,
};

use simple_error::SimpleError;

#[derive(Debug)]
pub struct ComplexDiceRoll {
    pub dice_roll: DiceRoll,
    pub roll_mods: Vec<RollMod>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct DiceRoll {
    pub number_of_dice: u64,
    pub dice_range: u64,
}

#[derive(PartialEq, Debug, Clone)]
pub struct RollMod {
    pub roll_mod_type: RollModType,
    pub value: u64,
}

#[derive(PartialEq, Debug, Clone)]
pub enum RollModType {
    E,
    R,
    K,
    L,
}

pub fn complex_dice_roll(input: &str) -> IResult<&str, ComplexDiceRoll> {
    let (input, _) = space0(input)?;
    let (input, dice_roll) = simple_dice_roll(input)?;
    let (input, roll_mods) = many0(dice_roll_mod)(input)?;

    Ok((
        input,
        ComplexDiceRoll {
            dice_roll,
            roll_mods,
        },
    ))
}

fn simple_dice_roll(input: &str) -> IResult<&str, DiceRoll> {
    let (input, (number_of_dice, _, dice_range)) =
        tuple((number, dice_roll_separator, number))(input)?;
    Ok((
        input,
        DiceRoll {
            number_of_dice,
            dice_range,
        },
    ))
}

fn dice_roll_mod(input: &str) -> IResult<&str, RollMod> {
    let (input, _) = space0(input)?;
    let (input, (roll_mod_type, value)) = tuple((roll_type, number))(input)?;
    Ok((
        input,
        RollMod {
            roll_mod_type,
            value,
        },
    ))
}

fn number(input: &str) -> IResult<&str, u64> {
    let (input, _) = space0(input)?;
    map_res(digit1, parse_number)(input)
}

fn roll_type(input: &str) -> IResult<&str, RollModType> {
    map_res(anychar, parse_roll_mod_type)(input)
}

fn dice_roll_separator(input: &str) -> IResult<&str, char> {
    let (input, _) = space0(input)?;
    one_of("dD")(input)
}

fn parse_number(text: &str) -> Result<u64, std::num::ParseIntError> {
    u64::from_str_radix(text, 10)
}

fn parse_roll_mod_type(c: char) -> Result<RollModType, SimpleError> {
    match &c.to_lowercase().next().unwrap() {
        'e' => Ok(RollModType::E),
        'r' => Ok(RollModType::R),
        'k' => Ok(RollModType::K),
        'l' => Ok(RollModType::L),
        _ => simple_error::bail!("Invalid Roll Type"),
    }
}

impl PartialEq<ComplexDiceRoll> for ComplexDiceRoll {
    fn eq(&self, other: &ComplexDiceRoll) -> bool {
        self.dice_roll == other.dice_roll
            && self.roll_mods.len() == other.roll_mods.len()
            && self.roll_mods.eq(&other.roll_mods)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_parser_parses_a_number() {
        assert_eq!(number("1234abcd"), Ok(("abcd", 1234u64)))
    }

    #[test]
    fn if_number_parser_doesnt_start_with_number_then_panic() {
        assert!(number("a123").is_err())
    }

    #[test]
    fn parse_dice_roll_works_as_expected() {
        assert_eq!(
            simple_dice_roll("23d45"),
            Ok((
                "",
                DiceRoll {
                    number_of_dice: 23,
                    dice_range: 45
                }
            ))
        );

        assert_eq!(
            simple_dice_roll(" 23 d  45"),
            Ok((
                "",
                DiceRoll {
                    number_of_dice: 23,
                    dice_range: 45
                }
            ))
        );

        assert_eq!(
            simple_dice_roll(" 23 D  45 "),
            Ok((
                " ",
                DiceRoll {
                    number_of_dice: 23,
                    dice_range: 45
                }
            ))
        );
    }

    #[test]
    fn parse_dice_roll_requires_the_dice_roll_to_have_the_d_character() {
        assert!(simple_dice_roll("123a123").is_err())
    }

    #[test]
    fn parse_dice_roll_with_additional_modifications_works_as_expected() {
        assert_eq!(
            complex_dice_roll("23d45e32"),
            Ok((
                "",
                ComplexDiceRoll {
                    dice_roll: DiceRoll {
                        number_of_dice: 23,
                        dice_range: 45
                    },
                    roll_mods: vec![RollMod {
                        roll_mod_type: RollModType::E,
                        value: 32
                    }],
                }
            ))
        );

        assert_eq!(
            complex_dice_roll("23d45 e32  R12 "),
            Ok((
                " ",
                ComplexDiceRoll {
                    dice_roll: DiceRoll {
                        number_of_dice: 23,
                        dice_range: 45
                    },
                    roll_mods: vec![
                        RollMod {
                            roll_mod_type: RollModType::E,
                            value: 32
                        },
                        RollMod {
                            roll_mod_type: RollModType::R,
                            value: 12
                        }
                    ],
                }
            ))
        );
    }
}