pub mod dice_roll;
pub mod math_ops;

extern crate nom;

use nom::{
    character::complete::{digit1, space0},
    combinator::map_res,
    IResult,
};

#[derive(Debug, Clone)]
pub struct ComplexDiceRoll {
    pub dice_roll: DiceRoll,
    pub roll_mods: Vec<RollMod>,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct DiceRoll {
    pub number_of_dice: u64,
    pub dice_range: u64,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct RollMod {
    pub roll_mod_type: RollModType,
    pub value: u64,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum RollModType {
    E,
    R,
    K,
    L,
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Operation {
    Add,
    Mul,
    Div,
    Sub,
}

impl PartialEq<ComplexDiceRoll> for ComplexDiceRoll {
    fn eq(&self, other: &ComplexDiceRoll) -> bool {
        self.dice_roll == other.dice_roll
            && self.roll_mods.len() == other.roll_mods.len()
            && self.roll_mods.eq(&other.roll_mods)
    }
}

fn parse_number(text: &str) -> Result<u64, std::num::ParseIntError> {
    u64::from_str_radix(text, 10)
}

fn number(input: &str) -> IResult<&str, u64> {
    let (input, _) = space0(input)?;
    map_res(digit1, parse_number)(input)
}
