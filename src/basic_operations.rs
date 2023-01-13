use super::aux_operations;

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
