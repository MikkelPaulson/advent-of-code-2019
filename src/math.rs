pub fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn lcm(a: i64, b: i64) -> i64 {
    if a == 0 && b == 0 {
        0
    } else {
        a * b / gcd(a, b)
    }
}
