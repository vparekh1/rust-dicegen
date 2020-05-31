extern crate rand;

use rand::Rng;
use std::iter;

struct AggregatedResult {
    kept_rolls: Vec<u64>,
    sum: u64,
    count: i64,
}

pub struct RollRequest<R: Rng> {
    number_of_rolls: u64,
    dice: u64,               // d
    explode_on: Option<u64>, // e
    result: Vec<u64>,
    rng: R
}

struct RollModifier {
    remove_count: Option<u64>,     // r
    top_limit: Option<u64>,        // t
    failure_limit: Option<u64>,    // f
    keep_count: Option<u64>,       // k
    keep_lower_count: Option<u64>, // kl
}

// impl AggregatedResult {
//     pub fn new(mut dice_rolls: Vec<u64>, modifiers: RollModifier) -> AggregatedResult {
//         dice_rolls.sort();
//         let kept_rolls = dice_rolls.iter().skip(modifiers.remove_count);

//         AggregatedResult {
//             kept_rolls: kept_rolls,
//             sum: 0,
//             count: 0,
//         }
//     }

// }

impl<R: Rng> RollRequest<R> {
    pub fn new(number_of_rolls: u64, dice: u64, explode_on: Option<u64>, rng: R) -> RollRequest<R> {
        RollRequest {
            number_of_rolls,
            dice,
            explode_on,
            result: vec![],
            rng
        }
    }

    pub fn roll_dice(&mut self) {
        let mut rolls_left = self.number_of_rolls;
        let mut result: Vec<u64> = vec![];
        while rolls_left > 0 {
            let dice_roll = self.rng.gen_range(1, self.dice + 1);
            result.push(dice_roll);
            if !self.should_explode(dice_roll) {
                rolls_left -= 1;
            }
        }

        self.result = result;
    }

    pub fn remove_count(&mut self, count: Option<u64>) {
        if count.is_some() {
            self.result = self.result[count.unwrap() as usize..].to_vec();
        }
    }

    pub fn keep_count(&mut self, count: Option<u64>) {
        if count.is_some() {
            self.result = self.result[count.unwrap() as usize..].to_vec();
        }
    }

    fn should_explode(&self, dice_roll: u64) -> bool {
        match self.explode_on {
            None => false,
            // If the explode number is 1 or less it'd go infinite.
            Some(e) => e > 1 && dice_roll >= e,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roll_request_returns_roll_result() {
        let rng = rand::rngs::mock::StepRng::new(2, 1);
        let mut request = RollRequest::new(5, 10, Some(5), rng);
        request.roll_dice();
        assert_eq!(request.result, vec![1, 1, 1, 1, 1]);

        // Works when you do it twice
        request.roll_dice();
        assert_eq!(request.result, vec![1, 1, 1, 1, 1]);
    }

    #[test]
    fn proper_number_of_elements_in_exploded_dice_roll() {
        let rng = rand::thread_rng();
        let mut request = RollRequest::new(10000, 1000, Some(500), rng);
        request.roll_dice();

        let expected_number_of_rolls = request.number_of_rolls
            + request.result.iter().fold(0, |acc, x| {
                acc + if *x >= request.explode_on.unwrap() {
                    1
                } else {
                    0
                }
            });
        assert_eq!(request.result.len() as u64, expected_number_of_rolls);
    }

    #[test]
    fn remove_count_removes_counts() {
        let rng = rand::rngs::mock::StepRng::new(2, 1);
        let mut request = RollRequest::new(5, 10, Some(5), rng);

        // Works when its Some(x)
        request.roll_dice();
        assert_eq!(request.result, vec![1, 1, 1, 1, 1]);
        request.remove_count(Some(2));
        assert_eq!(request.result, vec![1, 1, 1]);

        // Works when it's None
        request.roll_dice();
        assert_eq!(request.result, vec![1, 1, 1, 1, 1]);
        request.remove_count(None);
        assert_eq!(request.result, vec![1, 1, 1, 1, 1]);
    }
}
