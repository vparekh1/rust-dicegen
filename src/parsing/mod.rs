pub mod dice_roll;

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

impl PartialEq<ComplexDiceRoll> for ComplexDiceRoll {
    fn eq(&self, other: &ComplexDiceRoll) -> bool {
        self.dice_roll == other.dice_roll
            && self.roll_mods.len() == other.roll_mods.len()
            && self.roll_mods.eq(&other.roll_mods)
    }
}
