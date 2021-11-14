use std::mem;

fn main() {
    println!("gcd({}, {}) = {}", 1071, 1029, gcd(1071, 1029));
    println!("gcd({}, {}) = {}", 12, 30, gcd(12, 30));
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    if b > a {
        mem::swap(&mut a, &mut b);
    }

    while b != 0 {
        let old_b = b;
        b = a % b;
        a = old_b;
    }

    a
}
