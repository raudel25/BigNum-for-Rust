use super::Number;

pub fn sin(x: &Number, precision: usize) -> Number {
    sin_cos(x, true, precision)
}

pub fn cos(x: &Number, precision: usize) -> Number {
    sin_cos(x, false, precision)
}

fn sin_cos(x: &Number, sin: bool, precision: usize) -> Number {
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

    return result;
}
