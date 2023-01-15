use super::Number;

pub fn sin_cos(
    x: &Number,
    sin: bool,
    precision: usize,
    number0: Number,
    number1: Number,
) -> Number {
    let filter1 = if sin {
        |a| (a & 1) == 0
    } else {
        |a| (a & 1) != 0
    };
    let filter2 = if sin { |a| a % 4 == 1 } else { |a| a % 4 == 0 };

    let mut result = number0.clone();
    let mut pow_value = number1.clone();
    let mut fact = number1.clone();
    let mut index = number0.clone();

    //  Serie de taylor sen x cos x
    // https: // es.wikipedia.org / wiki / Serie_de_Taylor
    for i in 0..precision {
        if i != 0 {
            fact *= index.clone();
            pow_value *= x.clone();
        }

        index += number1.clone();

        if filter1(i) {
            continue;
        }

        if filter2(i) {
            result += pow_value.clone() / fact.clone();
        } else {
            result -= pow_value.clone() / fact.clone();
        }
    }

    result
}

pub fn atan(x: &Number, precision: usize, pi: Number, number0: Number, number1: Number) -> Number {
    //  arctan(x)+arctan(1/x)=pi/2
    //  arctan(1/x)=arccot(x)

    let number2 = number1.clone() + number1.clone();

    if x.abs() > number1 {
        return pi.clone()
            - atan(
                &(number1.clone() / x.clone()),
                precision,
                pi,
                number0,
                number1,
            );
    }
    let mut pow_value = x.clone();
    let mut index = number1.clone();
    let mut arctan = number0;
    let xx = x.clone() * x.clone();

    for i in 0..precision {
        let ind = 1 + i * 2;
        if ind % 4 == 1 {
            arctan += pow_value.clone() / index.clone();
        } else {
            arctan -= pow_value.clone() / index.clone();
        };
        pow_value *= xx.clone();
        index += number2.clone();
    }
    arctan
}

pub fn asin(x: &Number, precision: usize, number0: Number, number1: Number) -> Number {
    // if x_abs > number1:
    //     raise Exception("Operacion Invalida (arcsin recive valores entre 1 y -1)")

    let mut index = number1.clone();
    let mut even = number1.clone();
    let mut odd = number1.clone();
    let mut pow_value = number1.clone();
    let mut arcsin = number0.clone();

    for i in 1..precision {
        pow_value *= x.clone();

        if (i & 1) == 0 {
            even *= index.clone();
        };

        if (i & 1) == 1 {
            arcsin += (odd.clone() * pow_value.clone()) / (even.clone() * index.clone());
            odd *= index.clone();
        }
        index += number1.clone();
    }

    arcsin
}
