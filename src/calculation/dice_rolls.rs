extern crate rand;

use super::RollRequest;
use rand::Rng;

impl<R: Rng> RollRequest<R> {
    pub fn new(rng: R) -> RollRequest<R> {
        RollRequest { rng, result: None }
    }

    pub fn roll_dice<'a>(
        &'a mut self,
        number_of_dice: u64,
        dice_range: u64,
    ) -> &'a mut RollRequest<R> {
        let mut roll_result: Vec<u64> = vec![];
        for _ in 0..number_of_dice {
            let dice_roll = self.rng.gen_range(1, dice_range + 1);
            roll_result.push(dice_roll);
        }
        roll_result.sort();
        self.result = Some(roll_result);
        self
    }

    pub fn explode<'a>(
        &'a mut self,
        explode_on_greater: u64,
        dice_range: u64,
    ) -> &'a mut RollRequest<R> {
        if let Some(ref mut unwrapped_result) = self.result {
            let mut number_of_extra_rolls = unwrapped_result
                .iter()
                .filter(|x| **x >= explode_on_greater)
                .count();
            while number_of_extra_rolls > 0 {
                let dice_roll = self.rng.gen_range(1, dice_range + 1);
                if dice_roll < explode_on_greater {
                    number_of_extra_rolls -= 1;
                }
                unwrapped_result.push(dice_roll);
            }
            unwrapped_result.sort();
        }

        self
    }

    pub fn remove<'a>(&'a mut self, count: u64) -> &'a mut RollRequest<R> {
        if let Some(ref unwrapped_result) = self.result {
            if unwrapped_result.len() > count as usize {
                self.result = Some(unwrapped_result[count as usize..].to_vec());
            } else {
                self.result = None;
            }
        }
        self
    }

    pub fn keep<'a>(&'a mut self, count: u64) -> &'a mut RollRequest<R> {
        if let Some(ref unwrapped_result) = self.result {
            if unwrapped_result.len() > count as usize {
                let keep_index = unwrapped_result.len() - count as usize;
                self.result = Some(unwrapped_result[keep_index..].to_vec());
            }
        }
        self
    }

    pub fn keep_lower<'a>(&'a mut self, count: u64) -> &'a mut RollRequest<R> {
        if let Some(ref unwrapped_result) = self.result {
            if unwrapped_result.len() > count as usize {
                self.result = Some(unwrapped_result[..count as usize].to_vec());
            }
        }
        self
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn roll_request_returns_roll_result() {
        let rng = rand::rngs::mock::StepRng::new(2, 1);
        let mut request = RollRequest::new(rng);
        request.roll_dice(5, 10);
        assert_eq!(request.result, Some(vec![1, 1, 1, 1, 1]));

        // Works when you do it twice
        request.roll_dice(6, 10);
        assert_eq!(request.result, Some(vec![1, 1, 1, 1, 1, 1]));
    }

    #[test]
    fn proper_number_of_elements_in_exploded_dice_roll() {
        let rng = rand::thread_rng();
        let mut request = RollRequest::new(rng);

        let number_of_rolls = 10000;
        let dice_range = 1000;
        let explode_on_greater = 900;
        request
            .roll_dice(number_of_rolls, dice_range)
            .explode(explode_on_greater, dice_range);

        let expected_number_of_rolls = number_of_rolls
            + if let Some(ref unwrapped_request) = request.result {
                unwrapped_request.iter().fold(0, |acc, x| {
                    acc + if *x >= explode_on_greater { 1 } else { 0 }
                })
            } else {
                0
            };

        assert_eq!(
            request.result.unwrap().iter().count() as u64,
            expected_number_of_rolls
        );
    }

    #[test]
    fn remove_count_removes_counts() {
        let rng = rand::rngs::mock::StepRng::new(2, 1);
        let mut request = RollRequest::new(rng);

        // Works when its Some(x)
        request.roll_dice(5, 10).remove(2);
        assert_eq!(request.result, Some(vec![1, 1, 1]));

        // Works when its OOB
        request.roll_dice(5, 10).remove(5);
        assert_eq!(request.result, None);

        // Works when its OOB
        request.roll_dice(5, 10).remove(3).remove(6);
        assert_eq!(request.result, None);
    }

    #[test]
    fn lowest_results_are_removed() {
        let rng = rand::thread_rng();
        let mut request = RollRequest::new(rng);

        request.roll_dice(50, 100);
        let roll_result = request.result.clone().unwrap();
        request.remove(49);
        let removed_result = request.result.clone().unwrap();
        assert_eq!(*roll_result.iter().max().unwrap(), removed_result[0]);
    }

    #[test]
    fn keep_count_keeps_the_right_number_of_values() {
        let rng = rand::rngs::mock::StepRng::new(2, 1);
        let mut request = RollRequest::new(rng);

        // Works when its Some(x)
        request.roll_dice(5, 10).keep(2);
        assert_eq!(request.result, Some(vec![1, 1]));

        // Works when its OOB
        request.roll_dice(5, 10).keep(5);
        assert_eq!(request.result, Some(vec![1, 1, 1, 1, 1]));

        // Works when its OOB
        request.roll_dice(5, 10).keep(6);
        assert_eq!(request.result, Some(vec![1, 1, 1, 1, 1]));
    }

    #[test]
    fn highest_results_are_kept() {
        let rng = rand::thread_rng();
        let mut request = RollRequest::new(rng);

        request.roll_dice(50, 100);
        let roll_result = request.result.clone().unwrap();
        request.keep(1);
        let removed_result = request.result.clone().unwrap();
        assert_eq!(*roll_result.iter().max().unwrap(), removed_result[0]);
    }

    #[test]
    fn keep_lower_count_keeps_the_right_number_of_values() {
        let rng = rand::rngs::mock::StepRng::new(2, 1);
        let mut request = RollRequest::new(rng);

        // Works when its Some(x)
        request.roll_dice(5, 10).keep_lower(2);
        assert_eq!(request.result, Some(vec![1, 1]));

        // Works when its OOB
        request.roll_dice(5, 10).keep_lower(5);
        assert_eq!(request.result, Some(vec![1, 1, 1, 1, 1]));

        // Works when its OOB
        request.roll_dice(5, 10).keep_lower(6);
        assert_eq!(request.result, Some(vec![1, 1, 1, 1, 1]));
    }

    #[test]
    fn lowest_results_are_kept_in_keep_lower() {
        let rng = rand::thread_rng();
        let mut request = RollRequest::new(rng);

        request.roll_dice(50, 100);
        let roll_result = request.result.clone().unwrap();
        request.keep_lower(1);
        let removed_result = request.result.clone().unwrap();
        assert_eq!(*roll_result.iter().min().unwrap(), removed_result[0]);
    }
}
