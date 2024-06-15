use std::io::stdin;

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn extend_euclid(a: i64, b: i64, mut x: i64, mut y: i64) -> (i64, i64) {
    if b == 0 {
        x = 1i64;
        y = 0i64;
        return (x, y);
    }
    let (mut temp_x, mut temp_y) = extend_euclid(b, a % b, x, y);
    let temp = temp_x;
    temp_x = temp_y;
    temp_y = temp - (a / b) * temp_y;
    return (temp_x, temp_y);
}

fn main() {
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .ok()
        .expect("Failed to read line");
    let input: Vec<_> = input.trim().split(' ').collect();
    let mut A = input[0].parse::<i64>().unwrap();
    let mut B = input[1].parse::<i64>().unwrap();
    let mut C = input[2].parse::<i64>().unwrap();
    let mut C = -C;
    let g = gcd(A, B);
    if C % g != 0 {
        println!("-1");
        return;
    }
    A = A / g;
    B = B / g;
    C = C / g;
    let mut x = 0;
    let mut y = 0;
    let (x, y) = extend_euclid(A, B, x, y);
    println!("{} {}", x * C, y * C);
}
