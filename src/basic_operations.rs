use super::aux_operations;

pub fn sum_number(x: &Vec<u64>, y: &Vec<u64>, base10: u64) -> Vec<u64> {
    let tuple = aux_operations::equal_zeros_left_value(x, y);
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
    let tuple = aux_operations::equal_zeros_left_value(x, y);
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
    let aux = aux_operations::equal_zeros_left_value(x, y);
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
