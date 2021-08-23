pub fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let temp_a = a;
        a = b;
        b = temp_a % b;
    }

    a
}

pub fn lcm(a: u32, b: u32) -> u32 {
    a * b / gcd(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gcd_test() {
        assert_eq!(4, gcd(36, 20));
    }

    #[test]
    fn lcm_test() {
        assert_eq!(180, lcm(36, 20));
    }
}
