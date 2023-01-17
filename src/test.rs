#[cfg(test)]
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

    let a = big.num(&"0.25", true);
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

#[test]
fn pow_for_int() {
    let big = BigInt::new(17);

    let a = big.int_sytem(3);
    let b = big.int_sytem(81);

    assert_eq!(big.pow_int(&a, 4), b);
}
