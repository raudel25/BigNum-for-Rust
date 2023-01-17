use aux_operations::*;
use basic_operations::*;
use std::clone::Clone;
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, Sub, SubAssign};
use std::process;

mod aux_operations;
mod basic_operations;
mod logarithm;
mod pow_sqrt;
mod trigonometry;

pub struct BigNum {
    pub precision: usize,
    pub ind_base10: usize,
    base10: u128,
}

impl BigNum {
    pub fn new(precision: usize, ind_base10: usize) -> BigNum {
        Self::valid_assign(precision, ind_base10);

        BigNum {
            precision,
            ind_base10,
            base10: 10_u128.pow(ind_base10.try_into().unwrap()),
        }
    }

    pub fn num(&self, number: &String, positive: bool) -> Number {
        Number::new(
            number,
            self.precision,
            self.ind_base10,
            self.base10,
            positive,
            false,
        )
    }

    fn valid_assign(precision: usize, ind_base10: usize) {
        if ind_base10 > 17 {
            eprint!("Problem ind_base10 not supported: ind_base10 must be less than 9");
            process::exit(1);
        }
        if ind_base10 == 0 {
            eprint!("Problem ind_base10 not supported: ind_base10 cannot be 0");
            process::exit(1);
        }
        if precision == 0 {
            eprint!("Problem precision not supported: ind_base10 cannot be 0");
            process::exit(1);
        }
    }

    pub fn number0(&self) -> Number {
        Number::new_priv(
            &vec![0; self.precision + 1],
            self.precision,
            self.ind_base10,
            self.base10,
            true,
            false,
        )
    }

    pub fn number1(&self) -> Number {
        Number::new_priv(
            &[vec![0; self.precision], vec![1]].concat(),
            self.precision,
            self.ind_base10,
            self.base10,
            true,
            false,
        )
    }

    pub fn sin(&self, x: &Number) -> Number {
        trigonometry::sin_cos(x, true, 50, self.number0(), self.number1())
    }

    pub fn cos(&self, x: &Number) -> Number {
        trigonometry::sin_cos(x, false, 50, self.number0(), self.number1())
    }

    pub fn tan(&self, x: &Number) -> Number {
        self.sin(x) / self.cos(x)
    }

    pub fn cot(&self, x: &Number) -> Number {
        self.cos(x) / self.sin(x)
    }

    pub fn atan(&self, x: &Number) -> Number {
        trigonometry::atan(x, 1000, self.pi(), self.number0(), self.number1())
    }

    pub fn asin(&self, x: &Number) -> Number {
        let result =
            trigonometry::asin(x, 200, self.number0(), self.number1()).unwrap_or_else(|err| {
                eprintln!("Problem in asin: {}", err);
                process::exit(1);
            });

        result
    }

    pub fn acos(&self, x: &Number) -> Number {
        let result = self.pi() / (self.number1() + self.number1())
            - trigonometry::asin(x, 200, self.number1(), self.number0()).unwrap_or_else(|err| {
                eprintln!("Problem in asin: {}", err);
                process::exit(1);
            });

        result
    }

    pub fn pi(&self) -> Number {
        let number05 = Number::new(
            &"0.5".to_string(),
            self.precision,
            self.ind_base10,
            self.base10,
            true,
            false,
        );

        let number6 = Number::new_priv(
            &[vec![0; self.precision], vec![6]].concat(),
            self.precision,
            self.ind_base10,
            self.base10,
            true,
            false,
        );

        trigonometry::asin(&number05, 200, self.number0(), self.number1()).unwrap() * number6
    }

    pub fn e(&self) -> Number {
        let precision = 40;
        let mut e = self.number0();
        let mut fact = self.number1();
        let mut index = self.number0();

        // Formula de taylor e^x
        // https://es.wikipedia.org/wiki/Serie_de_Taylor
        for i in 0..precision {
            if i != 0 {
                fact *= index.clone();
            }
            e += self.number1() / fact.clone();
            index += self.number1();
        }

        e
    }

    pub fn ln(&self, x: &Number) -> Number {
        logarithm::ln_method(x, 150, self.number1(), self.number0())
    }

    pub fn log(&self, base: &Number, x: &Number) -> Number {
        logarithm::log_method(base, x, 150, self.number1(), self.number0())
    }

    pub fn pow_int(&self, x: &Number, ind: usize) -> Number {
        pow_sqrt::pow_numbers(x, ind, self.number1())
    }

    pub fn sqrt(&self, x: &Number, ind: usize) -> Number {
        let number_ind = Number::new(
            &ind.to_string(),
            self.precision,
            self.ind_base10,
            self.base10,
            true,
            false,
        );
        let number10 = Number::new_priv(
            &[vec![0; self.precision], vec![10]].concat(),
            self.precision,
            self.ind_base10,
            self.base10,
            true,
            false,
        );

        let x = x.clone();

        if ind == 1 {
            return x.clone();
        }
        if x == self.number0() {
            return self.number0();
        }

        let parity: bool = (ind & 1) == 0;
        let positive: bool = parity || x >= self.number0();

        let mut x = if x >= self.number0() { x } else { -x };

        let tuple = pow_sqrt::scalate_one(&x, number10.clone(), self.number1());
        x = tuple.0;
        if tuple.1 != 0 {
            x *= self.pow_int(&number10, ind - tuple.1 % ind);
        }
        if parity && !(x >= self.number0()) {
            eprintln!("Problem in operation sqrt: the result is not real");
            process::exit(1);
        }

        let mut result =
            pow_sqrt::algorithm_sqrt(&x, ind, &number_ind, 50, number10.clone(), self.number1());

        if tuple.1 != 0 {
            result /= self.pow_int(&number10, (tuple.1 + ind - tuple.1 % ind) / ind);
        }

        if !positive {
            result = -result
        }

        result
    }

    pub fn pow(&self, x: &Number, ind: &Number) -> Number {
        let ind = ind.clone();
        if ind == self.number0() {
            return self.number1();
        }

        let str_num = ind.abs().to_string();
        let part: Vec<&str> = str_num.split('.').collect();

        let denominator = 10_usize.pow(part[1].len().try_into().unwrap());
        let numerator = (part[0].to_string() + part[1]).parse().unwrap_or_else(|_| {
            eprintln!("Problem in operation pow: the exponent is not supported");
            process::exit(1);
        });

        let mut rets = 1;
        let mut x1 = numerator;
        let mut y1 = denominator;
        while rets != 0 {
            rets = x1 % y1;
            x1 = y1;
            y1 = rets;
        }

        let result = self.sqrt(x, denominator / x1);
        let result = self.pow_int(&result, numerator / x1);

        if ind >= self.number0() {
            result
        } else {
            self.number1() / result
        }
    }
}

pub struct BigInt {
    pub ind_base10: usize,
    base10: u128,
}

impl BigInt {
    pub fn new(ind_base10: usize) -> BigInt {
        BigNum::valid_assign(1, ind_base10);

        BigInt {
            ind_base10,
            base10: 10_u128.pow(ind_base10.try_into().unwrap()),
        }
    }

    pub fn int(&self, number: &String, positive: bool) -> Number {
        Number::new(number, 1, self.ind_base10, self.base10, positive, true)
    }
}

#[derive(Debug)]
pub struct Number {
    number_value: Vec<u128>,
    pub precision: usize,
    pub ind_base10: usize,
    pub base10: u128,
    pub positive: bool,
    pub int: bool,
}

impl Number {
    fn new(
        number: &String,
        precision: usize,
        ind_base10: usize,
        base10: u128,
        positive: bool,
        int: bool,
    ) -> Number {
        let number = int_and_decimal(number);
        let number = (
            eliminate_zeros_left(&number.0),
            eliminate_zeros_right(&number.1),
        );

        let number_value: Vec<u128> = eliminate_zeros_left_value(
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
            int,
        }
    }

    pub fn abs(&self) -> Number {
        Number::new_priv(
            &self.number_value,
            self.precision,
            self.ind_base10,
            self.base10,
            true,
            self.int,
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

    pub fn add(&self, rhs: &Self) -> Self {
        self.valid_operation(rhs);

        let x = self;
        let y = rhs;
        let lx = &x.number_value;
        let ly = &y.number_value;

        if x.positive == y.positive {
            return Number::new_priv(
                &sum_number(lx, ly, self.base10),
                self.precision,
                self.ind_base10,
                self.base10,
                x.positive,
                self.int,
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
                self.int,
            );
        }
        if compare == 1 {
            return Number::new_priv(
                &sub_number(lx, ly, self.base10),
                self.precision,
                self.ind_base10,
                self.base10,
                x.positive,
                self.int,
            );
        }

        return Number::new_priv(
            &sub_number(ly, lx, self.base10),
            self.precision,
            self.ind_base10,
            self.base10,
            y.positive,
            self.int,
        );
    }

    pub fn neg(&self) -> Self {
        Number::new_priv(
            &self.number_value,
            self.precision,
            self.ind_base10,
            self.base10,
            !self.positive,
            self.int,
        )
    }

    pub fn mul(&self, rhs: &Self) -> Self {
        self.valid_operation(rhs);

        let x = self;
        let y = rhs;

        let positive = x.positive == y.positive;

        let tuple = equal_zeros_left_value(&x.number_value, &y.number_value);

        let result = karatsuba_algorithm(&tuple.0, &tuple.1, self.base10);
        Number::new_priv(
            &result[self.precision..].iter().cloned().collect(),
            self.precision,
            self.ind_base10,
            self.base10,
            positive,
            self.int,
        )
    }

    pub fn div(&self, rhs: &Self) -> Self {
        self.valid_operation(rhs);

        let x = self;
        let y = rhs;
        let positive = x.positive == y.positive;

        let mut result = division_algorithm_d(
            &x.number_value,
            &y.number_value,
            self.precision,
            self.base10,
        )
        .unwrap_or_else(|err| {
            eprintln!("Problem in division: {}", err);
            process::exit(1);
        });

        if self.int {
            result[0] = 0;
        }

        Number::new_priv(
            &result,
            self.precision,
            self.ind_base10,
            self.base10,
            positive,
            self.int,
        )
    }

    fn new_priv(
        number: &Vec<u128>,
        precision: usize,
        ind_base10: usize,
        base10: u128,
        positive: bool,
        int: bool,
    ) -> Number {
        let number_value: Vec<u128> = eliminate_zeros_left_value(number, precision);

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
            int,
        }
    }

    fn my_string(&self) -> String {
        let sign_str = if !self.positive {
            String::from("-")
        } else {
            String::new()
        };

        let mut part_decimal = String::new();
        let mut part_int = String::new();

        for i in 0..self.precision {
            let aux = self.number_value[i].to_string();
            part_decimal = format!(
                "{}{}",
                add_zeros_left(&aux, self.ind_base10 - aux.len()),
                part_decimal
            );
        }
        for i in self.precision..self.number_value.len() {
            let aux = self.number_value[i].to_string();
            part_int = format!(
                "{}{}",
                add_zeros_left(&aux, self.ind_base10 - aux.len()),
                part_int
            );
        }

        part_decimal = eliminate_zeros_right(&part_decimal);
        part_int = eliminate_zeros_left(&part_int);

        format!("{}{}.{}", sign_str, part_int, part_decimal)
    }

    fn valid_operation(&self, other: &Self) {
        if self.precision != other.precision {
            eprintln!("Problem in operation: the do not have the same precision");
            process::exit(1);
        }
        if self.ind_base10 != other.ind_base10 {
            eprintln!("Problem in operation: the do not have the same base");
            process::exit(1);
        }
        if self.int != other.int {
            eprintln!("Problem in operation: operation between an integer an a real");
            process::exit(1);
        }
    }
}

impl Ord for Number {
    fn cmp(&self, other: &Self) -> Ordering {
        self.valid_operation(other);
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
        self.valid_operation(other);
        Self::compare_to(self, other) == 0
    }
}

impl Add for Number {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::add(&self, &rhs)
    }
}

impl AddAssign for Number {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self::add(&self, &rhs);
    }
}

impl Neg for Number {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::neg(&self)
    }
}

impl Sub for Number {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl SubAssign for Number {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self::add(&self, &Self::neg(&rhs));
    }
}

impl Mul for Number {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::mul(&self, &rhs)
    }
}

impl MulAssign for Number {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Self::mul(&self, &rhs);
    }
}

impl Div for Number {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::div(&self, &rhs)
    }
}

impl DivAssign for Number {
    fn div_assign(&mut self, rhs: Self) {
        *self = Self::div(&self, &rhs);
    }
}

impl Rem for Number {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        self.valid_operation(&rhs);
        if !self.int || !rhs.int {
            eprintln!("Problem in operation: operation remainder is not defined for a real");
            process::exit(1);
        }

        let aux = Self::mul(&Self::div(&self, &rhs), &rhs);
        self - aux
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.my_string())
    }
}

impl Clone for Number {
    fn clone(&self) -> Self {
        Number::new_priv(
            &self.number_value,
            self.precision,
            self.ind_base10,
            self.base10,
            self.positive,
            self.int,
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

    #[test]
    fn div() {
        let big = BigNum::new(4, 3);

        let a = big.num(&"1".to_string(), false);
        let b = big.num(&"10000".to_string(), true);
        let d = big.num(&"0.0001".to_string(), false);

        let c = a / b;
        assert_eq!(d, c);
    }

    #[test]
    fn str() {
        let big = BigNum::new(4, 6);

        let a = big.num(&"56".to_string(), false);
        let b = big.num(&"00789".to_string(), true);
        let d = big.num(&"0".to_string(), false);

        assert_eq!(a.to_string(), String::from("-56.0"));
        assert_eq!(b.to_string(), "789.0");
        assert_eq!(d.to_string(), "0.0");
    }

    #[test]
    fn assign() {
        let big = BigNum::new(4, 6);

        let mut a = big.num(&"56".to_string(), false);
        let b = big.num(&"789".to_string(), false);
        let d = big.num(&"845".to_string(), false);

        a += b;
        assert_eq!(a, d);
    }

    #[test]
    fn sin_cos() {
        let big = BigNum::new(6, 17);

        let a = big.num(&"0.3".to_string(), true);
        let x = 0.3_f64.sin();

        assert_eq!(big.sin(&a).to_string()[..10], x.to_string()[..10]);

        let a = big.num(&"2.23".to_string(), true);
        let x = 2.23_f64.sin();

        assert_eq!(big.sin(&a).to_string()[..10], x.to_string()[..10]);
    }

    #[test]
    fn asin_atan() {
        let big = BigNum::new(6, 17);

        let a = big.num(&"0.31".to_string(), true);
        let x = 0.31_f64.asin();

        assert_eq!(big.asin(&a).to_string()[..10], x.to_string()[..10]);

        let a = big.num(&"0.23".to_string(), true);
        let x = 0.23_f64.atan();

        assert_eq!(big.atan(&a).to_string()[..10], x.to_string()[..10]);
    }

    #[test]
    fn pi_e() {
        let big = BigNum::new(6, 17);

        let a = big.pi();

        assert_eq!(a.to_string()[..10], std::f64::consts::PI.to_string()[..10]);

        let a = big.e();

        assert_eq!(a.to_string()[..10], std::f64::consts::E.to_string()[..10]);
    }

    #[test]
    fn log() {
        let big = BigNum::new(6, 17);

        let a = big.num(&String::from("2.23"), true);

        assert_eq!(
            big.ln(&a).to_string()[..10],
            2.23_f64.ln().to_string()[..10]
        );

        let a = big.num(&String::from("3"), true);

        let b = big.num(&String::from("27"), true);

        assert_eq!(big.log(&a, &b), a);

        let a = big.num(&String::from("32.3"), true);

        let b = big.num(&String::from("27"), true);

        assert_eq!(
            big.log(&a, &b).to_string()[..4],
            27_f64.log(32.3).to_string()[..4]
        );
    }

    #[test]
    fn int() {
        let big = BigInt::new(17);

        let a = big.int(&"1".to_string(), true);
        let b = big.int(&"2".to_string(), true);
        let c = big.int(&"3".to_string(), true);

        assert_eq!(c / b, a);

        let a = big.int(&"0".to_string(), true);
        let b = big.int(&"2000000000".to_string(), true);
        let c = big.int(&"3".to_string(), true);

        assert_eq!(c / b, a);

        let a = big.int(&"1".to_string(), true);
        let b = big.int(&"3".to_string(), true);
        let c = big.int(&"10".to_string(), true);

        assert_eq!(c % b, a);
    }

    #[test]
    fn sqrt() {
        let big = BigNum::new(6, 17);

        let a = big.num(&"0.25".to_string(), true);
        let b = big.num(&"0.5".to_string(), true);
        let c = big.num(&"2".to_string(), true);
        let d = big.num(&"0.3".to_string(), true);

        assert_eq!(
            big.sqrt(&c, 2).to_string()[..10],
            2_f64.sqrt().to_string()[..10]
        );
        assert_eq!(big.sqrt(&a, 2), b);

        assert_eq!(
            big.sqrt(&d, 2).to_string()[..10],
            0.3_f64.sqrt().to_string()[..10]
        );
    }

    #[test]
    fn pow() {
        let big = BigNum::new(6, 17);

        let a = big.num(&"0.25".to_string(), true);
        let b = big.num(&"0.5".to_string(), true);
        let c = big.num(&"2".to_string(), true);

        assert_eq!(big.pow(&a, &b), b);

        assert_eq!(
            big.pow(&c, &b).to_string()[..10],
            2_f64.sqrt().to_string()[..10]
        );

        let b = big.num(&"0.5".to_string(), false);

        assert_eq!(
            big.pow(&c, &b).to_string()[..10],
            (1.0 / 2_f64.sqrt()).to_string()[..10]
        )
    }
}
