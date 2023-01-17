use super::Number;

pub fn ln_method(x: &Number, precision: usize, number1: Number, number0: Number) -> Number {
    // ln(1/x)=-ln(x)
    let positive = x.abs() > number1;
    let mut x = if positive {
        number1.clone() / x.clone()
    } else {
        x.clone()
    };
    x = number1.clone() - x;

    let mut pow_value = x.clone();
    let mut index = number1.clone();
    let mut ln = number0;

    // Serie de Taylor ln(1-x)
    // https://es.wikipedia.org/wiki/Serie_de_Taylor
    for _ in 1..precision {
        ln += pow_value.clone() / index.clone();
        pow_value *= x.clone();
        index += number1.clone()
    }

    if positive {
        ln
    } else {
        -ln
    }
}

pub fn log_method(
    x: &Number,
    y: &Number,
    precision: usize,
    number1: Number,
    number0: Number,
) -> Number {
    let mut pow_value = number1.clone();
    let mut index = number1.clone();

    if *y == number1 {
        return number0;
    }

    while pow_value <= *y {
        pow_value *= x.clone();
        if pow_value == *y {
            return index;
        }

        index += number1.clone()
    }

    ln_method(&y, precision, number1.clone(), number0.clone())
        / ln_method(&x, precision, number1, number0)
}
