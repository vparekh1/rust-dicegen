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
    dice: u64,                     // d
    explode_on: Option<u64>,       // e
    remove_count: Option<u64>,     // r
    top_limit: Option<u64>,        // t
    failure_limit: Option<u64>,    // f
    keep_count: Option<u64>,       // k
    keep_lower_count: Option<u64>, // kl
    result: Vec<u64>,
    rng: R,
}

impl<R: Rng> RollRequest<R> {
    pub fn new(
        number_of_rolls: u64,
        dice: u64,
        explode_on: Option<u64>,
        remove_count: Option<u64>,
        top_limit: Option<u64>,
        failure_limit: Option<u64>,
        keep_count: Option<u64>,
        keep_lower_count: Option<u64>,
        rng: R,
    ) -> RollRequest<R> {
        RollRequest {
            number_of_rolls,
            dice,
            explode_on,
            remove_count,
            top_limit,
            failure_limit,
            keep_count,
            keep_lower_count,
            result: vec![],
            rng,
        }
    }

    fn new_simple(
        number_of_rolls: u64,
        dice: u64,
        explode_on: Option<u64>,
        rng: R,
    ) -> RollRequest<R> {
        RollRequest {
            number_of_rolls,
            dice,
            explode_on,
            remove_count: None,
            top_limit: None,
            failure_limit: None,
            keep_count: None,
            keep_lower_count: None,
            result: vec![],
            rng,
        }
    }

    pub fn gen(&mut self) {
        self.roll_dice();
        self.remove(self.remove_count);
        self.keep(self.keep_count);
        self.keep_lower(self.keep_lower_count);
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

    fn remove(&mut self, count: Option<u64>) {
        if let Some(c) = count {
            if self.result.len() > c as usize {
                self.result = self.result[c as usize..].to_vec();
            } else {
                self.result = vec![];
            }
        }
    }

    fn keep(&mut self, count: Option<u64>) {
        if let Some(c) = count {
            if self.result.len() > c as usize {
                let keep_index = self.result.len() - c as usize;
                self.result = self.result[keep_index..].to_vec();
            }
        }
    }

    fn keep_lower(&mut self, count: Option<u64>) {
        if let Some(c) = count {
            if self.result.len() > c as usize {
                self.result = self.result[..c as usize].to_vec();
            }
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
        let mut request = RollRequest::new_simple(5, 10, Some(5), rng);
        request.roll_dice();
        assert_eq!(request.result, vec![1, 1, 1, 1, 1]);

        // Works when you do it twice
        request.roll_dice();
        assert_eq!(request.result, vec![1, 1, 1, 1, 1]);
    }

    #[test]
    fn proper_number_of_elements_in_exploded_dice_roll() {
        let rng = rand::thread_rng();
        let mut request = RollRequest::new_simple(10000, 1000, Some(500), rng);
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
        let mut request = RollRequest::new_simple(5, 10, Some(5), rng);

        // Works when its Some(x)
        request.roll_dice();
        assert_eq!(request.result, vec![1, 1, 1, 1, 1]);
        request.remove(Some(2));
        assert_eq!(request.result, vec![1, 1, 1]);

        // Works when it's None
        request.roll_dice();
        request.remove(None);
        assert_eq!(request.result, vec![1, 1, 1, 1, 1]);

        // Works when its OOB
        request.roll_dice();
        request.remove(Some(5));
        assert_eq!(request.result, vec![]);

        // Works when its OOB
        request.roll_dice();
        request.remove(Some(6));
        assert_eq!(request.result, vec![]);
    }

    #[test]
    fn keep_count_keeps_the_right_number_of_values() {
        let rng = rand::rngs::mock::StepRng::new(2, 1);
        let mut request = RollRequest::new_simple(5, 10, Some(5), rng);

        // Works when its Some(x)
        request.roll_dice();
        assert_eq!(request.result, vec![1, 1, 1, 1, 1]);
        request.keep(Some(2));
        assert_eq!(request.result, vec![1, 1]);

        // Works when it's None
        request.roll_dice();
        request.keep(None);
        assert_eq!(request.result, vec![1, 1, 1, 1, 1]);

        // Works when its OOB
        request.roll_dice();
        request.keep(Some(5));
        assert_eq!(request.result, vec![1, 1, 1, 1, 1]);

        // Works when its OOB
        request.roll_dice();
        request.keep(Some(6));
        assert_eq!(request.result, vec![1, 1, 1, 1, 1]);
    }

    #[test]
    fn keep_lower_count_keeps_the_right_number_of_values() {
        let rng = rand::rngs::mock::StepRng::new(2, 1);
        let mut request = RollRequest::new_simple(5, 10, Some(5), rng);

        // Works when its Some(x)
        request.roll_dice();
        assert_eq!(request.result, vec![1, 1, 1, 1, 1]);
        request.keep_lower(Some(2));
        assert_eq!(request.result, vec![1, 1]);

        // Works when it's None
        request.roll_dice();
        request.keep_lower(None);
        assert_eq!(request.result, vec![1, 1, 1, 1, 1]);

        // Works when its OOB
        request.roll_dice();
        request.keep_lower(Some(5));
        assert_eq!(request.result, vec![1, 1, 1, 1, 1]);

        // Works when its OOB
        request.roll_dice();
        request.keep_lower(Some(6));
        assert_eq!(request.result, vec![1, 1, 1, 1, 1]);
    }
}
