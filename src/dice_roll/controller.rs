extern crate rand;

use super::calculator;
use super::parser;

fn roll_complex_dice(roll: parser::ComplexDiceRoll) -> calculator::RollRequest<rand::prelude::ThreadRng> {
    let mut roll_request = roll_simple_dice(roll.dice_roll.clone());
    for i in roll.roll_mods {
        match i.roll_mod_type {
            parser::RollModType::E => roll_request.explode(i.value, roll.dice_roll.dice_range),
            parser::RollModType::R => roll_request.remove(i.value),
            parser::RollModType::K => roll_request.keep(i.value),
            parser::RollModType::L => roll_request.keep_lower(i.value),
        };
    }
    roll_request
}

fn parse_dice(text: &str) -> Result<parser::ComplexDiceRoll, &str> {
    match parser::complex_dice_roll(text) {
        Ok((_, successful_parsed_roll)) => Ok(successful_parsed_roll),
        Err(_) => Err("Invalid dice roll format"),
    }
}

fn roll_simple_dice(parser: parser::DiceRoll) -> calculator::RollRequest<rand::prelude::ThreadRng> {
    let rng = rand::thread_rng();
    let mut roll_request = calculator::RollRequest::new(rng);

    roll_request.roll_dice(parser.number_of_dice, parser.dice_range);
    roll_request
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_and_roll_dice(text: &str) -> Result<Vec<u64>, &str> {
        let dice_roll_parse = parse_dice(text)?;
        let roll_request = roll_complex_dice(dice_roll_parse);
        match roll_request.result {
            Some(result) => Ok(result),
            None => Err("No dice left to roll!"),
        }
    }

    #[test]
    fn dice_roll_calculates_as_expected() {
        assert_eq!(parse_and_roll_dice("17d1 k11 r6"), Ok(vec![1, 1, 1, 1, 1]));
    }
}
