extern crate rand;

use crate::calculation::RollRequest;
use crate::parsing::{ComplexDiceRoll, DiceRoll, RollModType};

pub fn roll_complex_dice(roll: ComplexDiceRoll) -> Option<Vec<u64>> {
    let mut roll_request = roll_simple_dice(roll.dice_roll.clone());
    for m in roll.roll_mods {
        match m.roll_mod_type {
            RollModType::E => roll_request.explode(m.value, roll.dice_roll.dice_range),
            RollModType::R => roll_request.remove(m.value),
            RollModType::K => roll_request.keep(m.value),
            RollModType::L => roll_request.keep_lower(m.value),
        };
    }
    roll_request.result
}

fn roll_simple_dice(parser: DiceRoll) -> RollRequest<rand::prelude::ThreadRng> {
    let rng = rand::thread_rng();
    let mut roll_request = RollRequest::new(rng);

    roll_request.roll_dice(parser.number_of_dice, parser.dice_range);
    roll_request
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_dice(text: &str) -> Result<ComplexDiceRoll, &str> {
        match crate::parsing::dice_roll::complex_dice_roll_parse(text) {
            Ok((_, successful_parsed_roll)) => Ok(successful_parsed_roll),
            Err(_) => Err("Invalid dice roll format"),
        }
    }

    fn parse_and_roll_dice(text: &str) -> Result<Vec<u64>, &str> {
        let dice_roll_parse = parse_dice(text)?;
        let roll_request = roll_complex_dice(dice_roll_parse);
        match roll_request {
            Some(result) => Ok(result),
            None => Err("No dice left to roll!"),
        }
    }

    #[test]
    fn dice_roll_calculates_as_expected() {
        assert_eq!(parse_and_roll_dice("17d1 k11 r6"), Ok(vec![1, 1, 1, 1, 1]));
    }
}
