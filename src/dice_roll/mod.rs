extern crate rand;

pub mod calculator;
pub mod parser;
pub mod controller;

#[derive(Debug, Clone)]
pub struct RollRequest<R: rand::Rng> {
    rng: R,
    result: Option<Vec<u64>>,
}

#[derive(Debug, Clone)]
pub struct ComplexDiceRoll {
    dice_roll: DiceRoll,
    roll_mods: Vec<RollMod>,
}

#[derive(PartialEq, Debug, Clone, Copy)]
struct DiceRoll {
    number_of_dice: u64,
    dice_range: u64,
}

#[derive(PartialEq, Debug, Clone, Copy)]
struct RollMod {
    roll_mod_type: RollModType,
    value: u64,
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum RollModType {
    E,
    R,
    K,
    L,
}

impl PartialEq<ComplexDiceRoll> for ComplexDiceRoll {
    fn eq(&self, other: &ComplexDiceRoll) -> bool {
        self.dice_roll == other.dice_roll
            && self.roll_mods.len() == other.roll_mods.len()
            && self.roll_mods.eq(&other.roll_mods)
    }
}