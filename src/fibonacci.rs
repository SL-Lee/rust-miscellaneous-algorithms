pub fn fibonacci_recursive(n: u128) -> u128 {
    if n == 0 || n == 1 {
        n
    } else {
        fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
    }
}

#[allow(unused_assignments)]
pub fn fibonacci_iterative(n: u128) -> u128 {
    if n == 0 {
        return 0;
    }

    let (mut second_previous_n, mut previous_n, mut current_n) = (0, 0, 1);

    for _ in 1..n {
        second_previous_n = previous_n;
        previous_n = current_n;
        current_n = second_previous_n + previous_n;
    }

    current_n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fibonacci_recursive_test() {
        assert_eq!(102334155, fibonacci_recursive(40));
    }

    #[test]
    fn fibonacci_iterative_test() {
        assert_eq!(102334155, fibonacci_iterative(40));
    }
}
