use aux_operations::*;

mod aux_operations;

pub struct BigNum {
    pub precision: usize,
    pub ind_base10: usize,
}

impl BigNum {
    pub fn new(precision: usize, ind_base10: usize) -> BigNum {
        BigNum {
            precision,
            ind_base10,
        }
    }

    pub fn num(&self, number: &String, positive: bool) -> Number {
        Number::new(number, self.precision, self.ind_base10, positive)
    }
}

pub struct Number {
    number_value: Vec<u64>,
    pub precision: usize,
    pub ind_base10: usize,
    pub positive: bool,
}

impl Number {
    pub fn new(number: &String, precision: usize, ind_base10: usize, positive: bool) -> Number {
        let number = int_and_decimal(number);
        let number = (
            eliminate_zeros_left(&number.0),
            eliminate_zeros_right(&number.1),
        );

        let number_value: Vec<u64> = eliminate_zeros_left_value(
            &create_number_value(&number, precision, ind_base10),
            precision,
        );

        let positive = if check_zero(&number_value) {
            true
        } else {
            positive
        };

        Number {
            number_value,
            precision,
            ind_base10,
            positive,
        }
    }

    pub fn abs(&self) -> Number {
        Number::new_priv(&self.number_value, self.precision, self.ind_base10, true)
    }

    fn new_priv(number: &Vec<u64>, precision: usize, ind_base10: usize, positive: bool) -> Number {
        let number_value: Vec<u64> = eliminate_zeros_left_value(number, precision);

        let positive = if check_zero(&number_value) {
            true
        } else {
            positive
        };

        Number {
            number_value,
            precision,
            ind_base10,
            positive,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn num() {
        let big = BigNum::new(1, 2);

        let a = big.num(&"233".to_string(), false);

        let b = big.num(&"0000".to_string(), false);

        println!("{}", a.number_value.len());

        assert_eq!(b.number_value.len(), 2);
        assert!(a.abs().positive);
        assert!(b.positive);
    }
}
