pub fn binary_search<T>(array: &[T], q: T) -> Option<usize>
where
    T: PartialEq + PartialOrd,
{
    let mut low = 0;
    let mut high = array.len() - 1;

    while low <= high {
        let mut mid = (low + high) / 2;

        if array[mid] > q {
            high = mid - 1;
        } else if array[mid] < q {
            low = mid + 1;
        } else {
            while mid > 0 && array[mid - 1] == q {
                mid -= 1;
            }

            return Some(mid);
        }
    }

    None
}

pub fn binary_search_by_key<T, U, F>(array: &[T], q: U, key: F) -> Option<usize>
where
    U: PartialEq + PartialOrd,
    F: Fn(&T) -> U,
{
    let mut low = 0;
    let mut high = array.len() - 1;

    while low <= high {
        let mut mid = (low + high) / 2;

        if key(&array[mid]) > q {
            high = mid - 1;
        } else if key(&array[mid]) < q {
            low = mid + 1;
        } else {
            while mid > 0 && key(&array[mid - 1]) == q {
                mid -= 1;
            }

            return Some(mid);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_search_test() {
        let array = vec![0, 1, 1, 2, 2, 3, 4];
        assert_eq!(Some(3), binary_search(&array, 2));
    }

    #[test]
    fn binary_search_by_key_test() {
        struct Wrapper(i32);

        let array = vec![
            Wrapper(0),
            Wrapper(1),
            Wrapper(1),
            Wrapper(2),
            Wrapper(2),
            Wrapper(3),
            Wrapper(4),
        ];
        assert_eq!(Some(3), binary_search_by_key(&array, 2, |x| x.0));
    }
}
