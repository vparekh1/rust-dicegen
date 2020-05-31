extern crate nom;

fn parse_dice_string<'a>(dice_string: &'a str) -> Result<Vec<&'a str>, &str> {
    
    Ok(vec![dice_string])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_value_returns_single_value_in_vector_form() {
        assert_eq!(Ok(vec!["1"]), parse_dice_string("1"));
    }

    // #[test]
    // fn if_first_character_is_not_number_then_fail() {
    //     assert_eq!(Err("Must start with number"), parse_dice_string("a"));
    // }
}
