use super::Number;

pub fn pow_numbers(x: &Number, y: usize, number1: Number) -> Number {
    let mut result = number1;
    for _ in 0..y {
        result *= x.clone();
    }

    result
}

pub fn algorithm_sqrt(
    x: &Number,
    y: usize,
    number_y: &Number,
    precision: usize,
    number10: Number,
    number1: Number,
) -> Number {
    let tuple = approximate_integer(x, y, number10, number1.clone());
    let mut value = tuple.0;

    if tuple.1 {
        return value;
    }

    let value_y = number_y.clone();
    let value_y_1 = value_y.clone() - number1.clone();

    //  https: // es.frwiki.wiki/wiki/Algorithme_de_calcul_de_la_racine_n-i % C3 % A8me
    for _ in 0..precision {
        let aux = x.clone() / pow_numbers(&value, y - 1, number1.clone());
        value = number1.clone() / value_y.clone() * (value_y_1.clone() * value + aux);
    }

    value
}

pub fn approximate_integer(
    x: &Number,
    y: usize,
    number10: Number,
    number1: Number,
) -> (Number, bool) {
    let x = x.clone();
    let mut value = number10.clone();
    while value < x {
        value *= number10.clone();
    }

    if value > x {
        value = value / number10;
    }

    let mut pow_value = pow_numbers(&value, y, number1.clone());

    while pow_value < x {
        value += number1.clone();
        pow_value = pow_numbers(&value, y, number1.clone());
    }

    if pow_value == x {
        (value, true)
    } else {
        (value - number1, false)
    }
}

pub fn scalate_one(x: &Number, number10: Number, number1: Number) -> (Number, usize) {
    let mut cant = 0;
    let mut x = x.clone();

    while x < number1 {
        x *= number10.clone();
        cant += 1;
    }

    (x, cant)
}
