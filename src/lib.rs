use aux_operations::*;
use basic_operations::*;
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::ops::{Add, Div, Mul, Neg, Sub};

mod aux_operations;
mod basic_operations;

pub struct BigNum {
    pub precision: usize,
    pub ind_base10: usize,
    pub base10: u64,
}

impl BigNum {
    pub fn new(precision: usize, ind_base10: usize) -> BigNum {
        BigNum {
            precision,
            ind_base10,
            base10: 10_u64.pow(ind_base10.try_into().unwrap()),
        }
    }

    pub fn num(&self, number: &String, positive: bool) -> Number {
        Number::new(
            number,
            self.precision,
            self.ind_base10,
            self.base10,
            positive,
        )
    }
}

pub struct Number {
    number_value: Vec<u64>,
    pub precision: usize,
    pub ind_base10: usize,
    pub base10: u64,
    pub positive: bool,
}

impl Number {
    fn new(
        number: &String,
        precision: usize,
        ind_base10: usize,
        base10: u64,
        positive: bool,
    ) -> Number {
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
            base10,
            ind_base10,
            positive,
        }
    }

    pub fn abs(&self) -> Number {
        Number::new_priv(
            &self.number_value,
            self.precision,
            self.ind_base10,
            self.base10,
            true,
        )
    }

    fn compare_to(&self, n: &Number) -> i32 {
        if self.positive == n.positive {
            if self.positive {
                return compare_list(&self.number_value, &n.number_value);
            }
            return compare_list(&n.number_value, &self.number_value);
        }
        if self.positive {
            return 1;
        }

        return -1;
    }

    fn new_priv(
        number: &Vec<u64>,
        precision: usize,
        ind_base10: usize,
        base10: u64,
        positive: bool,
    ) -> Number {
        let number_value: Vec<u64> = eliminate_zeros_left_value(number, precision);

        let positive = if check_zero(&number_value) {
            true
        } else {
            positive
        };

        Number {
            number_value,
            precision,
            base10,
            ind_base10,
            positive,
        }
    }
}

impl Ord for Number {
    fn cmp(&self, other: &Self) -> Ordering {
        0.cmp(&(-self.compare_to(other)))
    }
}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Number {}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        Self::compare_to(self, other) == 0
    }
}

impl Add for Number {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let x = &self;
        let y = &rhs;
        let lx = &x.number_value;
        let ly = &y.number_value;

        if x.positive == y.positive {
            return Number::new_priv(
                &sum_number(lx, ly, self.base10),
                self.precision,
                self.ind_base10,
                self.base10,
                x.positive,
            );
        };

        let compare = x.abs().compare_to(&y.abs());
        if compare == 0 {
            return Number::new_priv(
                &vec![0; self.precision + 1],
                self.precision,
                self.ind_base10,
                self.base10,
                x.positive,
            );
        }
        if compare == 1 {
            return Number::new_priv(
                &sub_number(lx, ly, self.base10),
                self.precision,
                self.ind_base10,
                self.base10,
                x.positive,
            );
        }

        return Number::new_priv(
            &sub_number(ly, lx, self.base10),
            self.precision,
            self.ind_base10,
            self.base10,
            y.positive,
        );
    }
}

impl Neg for Number {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Number::new_priv(
            &self.number_value,
            self.precision,
            self.ind_base10,
            self.base10,
            !self.positive,
        )
    }
}

impl Sub for Number {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl Mul for Number {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let x = &self;
        let y = &rhs;

        let positive = x.positive == y.positive;

        let tuple = equal_zeros_left_value(&x.number_value, &y.number_value);

        let result = karatsuba_algorithm(&tuple.0, &tuple.1, self.base10);
        Number::new_priv(
            &result[self.precision..].iter().cloned().collect(),
            self.precision,
            self.ind_base10,
            self.base10,
            positive,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn num() {
        let big = BigNum::new(1, 2);

        let a = big.num(&"233".to_string(), true);

        let a1 = big.num(&"2333".to_string(), false);

        let b = big.num(&"0000".to_string(), false);

        println!("{}", a.number_value.len());

        assert_eq!(b.number_value.len(), 2);
        assert!(a.abs().positive);
        assert!(b.positive);

        assert!(b > a1);
        // assert!(a1 < a);
    }

    #[test]
    fn sum_sub() {
        let big = BigNum::new(4, 1);

        let a = big.num(&"100".to_string(), false);
        let b = big.num(&"100".to_string(), true);
        let d = big.num(&"0".to_string(), false);

        let c = a + b;
        assert!(-d == c);
    }

    #[test]
    fn mul() {
        let big = BigNum::new(4, 1);

        let a = big.num(&"2".to_string(), false);
        let b = big.num(&"234".to_string(), true);
        let d = big.num(&"468".to_string(), false);

        let c = a * b;
        assert!(d == c);
    }
}
