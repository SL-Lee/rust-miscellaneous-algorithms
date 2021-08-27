pub fn factorial_recursive(n: u128) -> u128 {
    match n {
        0 | 1 => 1,
        _ => n * factorial_recursive(n - 1),
    }
}

pub fn factorial_for_loop(n: u128) -> u128 {
    let mut factorial = 1;

    for i in 1..n + 1 {
        factorial *= i;
    }

    factorial
}

pub fn factorial_iterator(n: u128) -> u128 {
    (1..n + 1).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_recursive_test() {
        assert_eq!(265252859812191058636308480000000, factorial_recursive(30));
    }

    #[test]
    fn factorial_for_loop_test() {
        assert_eq!(265252859812191058636308480000000, factorial_for_loop(30));
    }

    #[test]
    fn factorial_iterator_test() {
        assert_eq!(265252859812191058636308480000000, factorial_iterator(30));
    }
}
