use diceroll::controller::parse_and_roll_dice;

fn main() {
    let dice_roll = std::env::args().nth(1).expect("");
    let v2 = parse_and_roll_dice(&dice_roll).unwrap();
    println!("{:?}", v2);
}