use super::Number;

pub fn sin_cos(x: &Number, sin: bool, precision: usize) -> Number {
    let filter1 = if sin {
        |a| (a & 1) == 0
    } else {
        |a| (a & 1) != 0
    };
    let filter2 = if sin { |a| a % 4 == 1 } else { |a| a % 4 == 0 };

    let mut result = x.number0();
    let mut pow_value = x.number1();
    let mut fact = x.number1();
    let mut index = x.number0();

    //  Serie de taylor sen x cos x
    // https: // es.wikipedia.org / wiki / Serie_de_Taylor
    for i in 0..precision {
        if i != 0 {
            fact *= index.clone();
            pow_value *= x.clone();
        }

        index += x.number1();

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

pub fn atan(x: &Number, precision: usize, pi: Number) -> Number {
    //  arctan(x)+arctan(1/x)=pi/2
    //  arctan(1/x)=arccot(x)

    let number2 = x.number1() + x.number1();

    if x.abs() > x.number1() {
        return pi.clone() - atan(&(x.number1() / x.clone()), precision, pi);
    }
    let mut pow_value = x.clone();
    let mut index = x.number1();
    let mut arctan = x.number0();
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

pub fn asin(x: &Number, precision: usize) -> Number {
    // if x_abs > number1:
    //     raise Exception("Operacion Invalida (arcsin recive valores entre 1 y -1)")

    let mut index = x.number1();
    let mut even = x.number1();
    let mut odd = x.number1();
    let mut pow_value = x.number1();
    let mut arcsin = x.number0();

    for i in 1..precision {
        pow_value *= x.clone();

        if (i & 1) == 0 {
            even *= index.clone();
        };

        if (i & 1) == 1 {
            arcsin += (odd.clone() * pow_value.clone()) / (even.clone() * index.clone());
            odd *= index.clone();
        }
        index += x.number1();
    }

    arcsin
}
