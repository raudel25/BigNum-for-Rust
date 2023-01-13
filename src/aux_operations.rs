pub fn int_and_decimal(number: &String) -> (String, String) {
    let part: Vec<&str> = number.split(".").collect();

    (
        part[0].to_string(),
        if part.len() == 2 {
            part[1].to_string()
        } else {
            String::from("0")
        },
    )
}

pub fn check_zero(number_value: &Vec<u64>) -> bool {
    for i in number_value {
        if i.clone() != 0 {
            return false;
        }
    }

    true
}

pub fn create_number_value(
    number: &(String, String),
    precision: usize,
    ind_base10: usize,
) -> Vec<u64> {
    let mut number_value: Vec<u64> = vec![0; precision + 1];
    let len = precision + 1;

    let part_int = add_zeros_left(&number.0, ind_base10 - number.0.len() % ind_base10);
    let part_decimal = add_zeros_right(&number.1, ind_base10 - number.1.len() % ind_base10);

    for i in 0..(part_decimal.len() / ind_base10) {
        number_value[len - i - 2] = part_decimal[i * ind_base10..ind_base10 * (i + 1)]
            .parse()
            .unwrap();

        if i + 1 == precision {
            break;
        }
    }

    number_value[len - 1] = part_int[part_int.len() - ind_base10..part_int.len()]
        .parse()
        .unwrap();

    for i in 1..part_int.len() / ind_base10 {
        number_value.push(
            part_int[part_int.len() - (1 + i) * ind_base10..part_int.len() - i * ind_base10]
                .parse()
                .unwrap(),
        );
    }

    number_value
}

fn add_zeros_left(s: &String, cant: usize) -> String {
    format!("{}{}", &create_zeros(cant), s)
}

fn add_zeros_right(s: &String, cant: usize) -> String {
    format!("{}{}", s, &create_zeros(cant))
}
fn create_zeros(cant: usize) -> String {
    let mut s = String::new();

    for _ in 0..cant {
        s.push('0');
    }

    s
}

pub fn eliminate_zeros_left(s: &String) -> String {
    let mut ind = 0;
    let mut find = false;

    for i in s.chars().enumerate() {
        if i.1 != '0' {
            ind = i.0;
            find = true;
            break;
        }
    }

    if !find {
        return String::from("0");
    }

    s[ind..s.len()].to_string()
}

pub fn eliminate_zeros_right(s: &String) -> String {
    let s: String = s.chars().rev().collect();

    eliminate_zeros_left(&s)
}

// def equal_zeros_left(x: str, y: str) -> tuple:
//     ind: int = max(len(x), len(y))

//     return ind, add_zeros_left(x, ind - len(x)), add_zeros_left(y, ind - len(y))

// def equal_zeros_right(x: str, y: str) -> tuple:
//     ind: int = max(len(x), len(y))

//     return ind, add_zeros_right(x, ind - len(x)), add_zeros_right(y, ind - len(y))

pub fn eliminate_zeros_left_value(number_value: &Vec<u64>, precision: usize) -> Vec<u64> {
    let mut l = Vec::new();
    let mut act = false;

    for i in (0..number_value.len()).rev() {
        if number_value[i] != 0 {
            act = true;
        }
        //reivsar
        if i == precision {
            act = true;
        }

        if act {
            l.push(number_value[i]);
        }
    }

    l.reverse();
    return l;
}

fn add_zeros_left_value(number_value: &Vec<u64>, cant: usize) -> Vec<u64> {
    let mut l = number_value.clone();

    for _ in 0..cant {
        l.push(0)
    }

    l
}

fn add_zeros_right_value(number_value: &Vec<u64>, cant: usize) -> Vec<u64> {
    let l = vec![0; cant];

    [l, number_value.clone()].concat()
}

pub fn equal_zeros_left_value(x: &Vec<u64>, y: &Vec<u64>) -> (Vec<u64>, Vec<u64>) {
    let lx = add_zeros_left_value(x, x.len().max(y.len()) - x.len());
    let ly = add_zeros_left_value(y, x.len().max(y.len()) - y.len());

    (lx, ly)
}
