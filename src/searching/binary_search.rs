use std::cmp::Ordering;

pub fn binary_search<T: Ord>(item: &T, arr: &[T]) -> Option<usize> {
    let mut left = 0;
    let mut right = 0;
    while left < right {
        let mid = left + (right - left) / 2;

        match item.cmp(&arr[mid]) {
            Ordering::Less => right = mid,
            Ordering::Equal => return Some(mid),
            Ordering::Greater => left = mid + 1,
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_search_strings() {
        let index = binary_search(&"a", &vec!["a", "b", "c", "d"]);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn binary_search_ints() {
        struct Tests {
            ds: Vec<i32>,
            cases: Vec<(i32, Option<usize>)>,
        }
        let tests = Tests {
            ds: vec![1, 2, 3, 4],
            cases: vec![(1, Some(0)), (2, Some(1)), (3, Some(2)), (4, Some(3))],
        };

        for test in tests.cases.iter() {
            let index = binary_search(&test.0, &tests.ds);
            assert_eq!(index, test.1);
        }
    }

    #[test]
    fn binary_search_not_found() {
        let index = binary_search(&5, &vec![1, 2, 3, 4]);
        assert_eq!(index, None);
    }

    #[test]
    fn binary_search_empty() {
        let index = binary_search(&1, &vec![]);
        assert_eq!(index, None);
    }
}
