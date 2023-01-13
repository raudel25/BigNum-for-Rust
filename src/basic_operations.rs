use super::aux_operations::add_zeros_right_value;
use super::aux_operations::equal_zeros_left_value;

pub fn sum_number(x: &Vec<u64>, y: &Vec<u64>, base10: u64) -> Vec<u64> {
    let tuple = equal_zeros_left_value(x, y);
    let x = tuple.0;
    let y = tuple.1;

    let mut result = Vec::new();
    let mut drag = 0;

    for i in 0..x.len() {
        let mut n = x[i] + y[i];

        n += drag;
        drag = n / base10;
        result.push(n % base10);
    }

    if drag != 0 {
        result.push(drag)
    }

    result
}

pub fn sub_number(x: &Vec<u64>, y: &Vec<u64>, base10: u64) -> Vec<u64> {
    let tuple = equal_zeros_left_value(x, y);
    let x = tuple.0;
    let y = tuple.1;

    let mut result: Vec<u64> = Vec::new();
    let mut drag = 0;
    let base10: i64 = base10.try_into().unwrap();

    for i in 0..x.len() {
        let x1: i64 = x[i].try_into().unwrap();
        let y1: i64 = y[i].try_into().unwrap();
        let mut n: i64 = x1 - y1;

        n = n - drag;
        drag = if n < 0 { 1 } else { 0 };
        n = if n < 0 { n + base10 } else { n };
        result.push(n.try_into().unwrap());
    }

    result
}

pub fn compare_list(x: &Vec<u64>, y: &Vec<u64>) -> i32 {
    let aux = super::aux_operations::equal_zeros_left_value(x, y);
    let x = aux.0;
    let y = aux.1;

    for i in (0..x.len()).rev() {
        if x[i] > y[i] {
            return 1;
        }
        if y[i] > x[i] {
            return -1;
        }
    }

    return 0;
}

fn simple_multiplication(x: &Vec<u64>, y: u64, base10: u64) -> Vec<u64> {
    let mut drag = 0;
    let mut result = Vec::new();

    for i in x {
        let n = i * y + drag;

        drag = n / base10;
        result.push(n % base10);
    }

    if drag != 0 {
        result.push(drag);
    }

    return result;
}

pub fn karatsuba_algorithm(x: &Vec<u64>, y: &Vec<u64>, base10: u64) -> Vec<u64> {
    let tuple = super::aux_operations::equal_zeros_left_value(&x, &y);
    let x = tuple.0;
    let y = tuple.1;

    if x.len() == 1 {
        return vec![x[0] * y[0] % base10, x[0] * y[0] / base10];
    }

    // Algortimo de Karatsuba
    // https: // es.wikipedia.org/wiki/Algoritmo_de_Karatsuba

    let n = x.len() / 2;

    let x0 = &x[..n].iter().cloned().collect();
    let x1 = &x[n..x.len()].iter().cloned().collect();
    let y0 = &y[..n].iter().cloned().collect();
    let y1 = &y[n..y.len()].iter().cloned().collect();

    let z2 = add_zeros_right_value(&karatsuba_algorithm(x1, y1, base10), 2 * n);
    let z11 = add_zeros_right_value(&karatsuba_algorithm(x1, y0, base10), n);
    let z12 = add_zeros_right_value(&karatsuba_algorithm(y1, x0, base10), n);
    let z1 = sum_number(&z11, &z12, base10);
    let z0 = karatsuba_algorithm(&x0, &y0, base10);

    return sum_number(&z2, &sum_number(&z1, &z0, base10), base10);
}
