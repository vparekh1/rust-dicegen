use super::{
    Operation, Operation::Add, Operation::Div, Operation::Mul, Operation::Sub, RollScalar,
    RollScalar::Number, RollScalar::Roll, TwoScalarCalculation,
};

fn scalar_calculate(scalar_calc: &TwoScalarCalculation) -> f64 {
    simple_calculate(
        &scalar_calc.first,
        &scalar_calc.second,
        scalar_calc.operation,
    )
}

fn roll_scalar_to_float(roll_scalar: &RollScalar) -> f64 {
    match roll_scalar {
        Number(num) => *num,
        Roll(rolls) => rolls
            .as_ref()
            .map_or(0f64, |r| r.iter().fold(0f64, |sum, x| sum + *x as f64)),
    }
}

fn simple_calculate(first: &RollScalar, second: &RollScalar, op: Operation) -> f64 {
    let first_scalar = roll_scalar_to_float(first);
    let second_scalar = roll_scalar_to_float(second);
    match op {
        Add => first_scalar + second_scalar,
        Mul => first_scalar * second_scalar,
        Div => first_scalar / second_scalar,
        Sub => first_scalar - second_scalar,
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn simple_scalar_calculation_works_as_expected() {
        let n5: RollScalar = Number(5.0);
        let n9: RollScalar = Number(9.0);
        let v10: RollScalar = Roll(Some(vec![1, 2, 3, 4]));
        let v26: RollScalar = Roll(Some(vec![5, 6, 7, 8]));
        let v_none: RollScalar = Roll(None);

        assert_eq!(simple_calculate(&n5, &n9, Add), 14.0);
        assert_eq!(simple_calculate(&n5, &v10, Add), 15.0);
        assert_eq!(simple_calculate(&v26, &n9, Add), 35.0);
        assert_eq!(simple_calculate(&v26, &v10, Add), 36.0);
        assert_eq!(simple_calculate(&v_none, &v10, Add), 10.0);
        assert_eq!(simple_calculate(&n5, &v_none, Add), 5.0);
        assert_eq!(simple_calculate(&v_none, &n9, Add), 9.0);
        assert_eq!(simple_calculate(&v26, &v_none, Add), 26.0);

        assert_eq!(simple_calculate(&n5, &n9, Mul), 45.0);
        assert_eq!(simple_calculate(&n5, &n9, Div), 5.0 / 9.0);
        assert_eq!(simple_calculate(&n5, &n9, Sub), -4.0);
    }

    #[test]
    fn scalar_calculate_test() {
        let tsc = TwoScalarCalculation {
            operation: Sub,
            first: Roll(Some(vec![1, 2, 3, 4])),
            second: Number(9.0),
        };

        assert_eq!(scalar_calculate(&tsc), 1.0);
    }
}
