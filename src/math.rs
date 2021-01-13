pub fn gcd(a: isize, b: isize) -> isize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn lcm(a: isize, b: isize) -> isize {
    if a == 0 && b == 0 {
        0
    } else {
        a * b / gcd(a, b)
    }
}
