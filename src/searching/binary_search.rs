use std::cmp::Ordering;

/// binary search is a algorithm for finding an element within a sorted collection.
///
/// # Examples
///
/// ```
/// use yumorithm::searching;
///
/// assert_eq!(searching::binary_search(&10, &vec![1, 2, 3]), None);
/// assert_eq!(searching::binary_search(&1, &vec![1, 2, 3]), Some(0));
/// ```
///
/// In the following example iterations to find 7 in the given array are illustrated:
/// ┌──────────────────────────┐
/// │  -5 -2 0 1 2 4 5 6 7 10  │   search item = 7
/// └──┬─────────┬─────────┬───┘
///    │         │         │
///    │        mid        │
///   low                 high
///
/// ┌──────────────────────────┐
/// │              4 5 6 7 10  │
/// └──────────────┬───┬───┬───┘
///                │   │   │
///                │  mid  │
///               low     high
///
/// ┌──────────────────────────┐
/// │                  6 7 10  │
/// └──────────────────┬─┬─┬───┘
///                    │ │ │
///                    │ │ │
///                   low│high
///                      └─► mid
pub fn binary_search<T: Ord>(item: &T, arr: &[T]) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();
    while left < right {
        let mid = left + ((right - left) / 2);

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
    fn binary_search_strings_test() {
        let index = binary_search(&"a", &vec!["a", "b", "c", "d"]);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn binary_search_ints_test() {
        struct Tests {
            ds: Vec<i32>,
            cases: Vec<(i32, Option<usize>)>,
        }
        let tests = Tests {
            ds: vec![1, 2, 3, 4],
            cases: vec![
                (1, Some(0)), // (input, wanted)
                (2, Some(1)),
                (3, Some(2)),
                (4, Some(3)),
            ],
        };

        for t in tests.cases.iter() {
            let index = binary_search(&t.0, &tests.ds);
            assert_eq!(index, t.1);
        }
    }

    #[test]
    fn binary_search_not_found_test() {
        let index = binary_search(&5, &vec![1, 2, 3, 4]);
        assert_eq!(index, None);
    }

    #[test]
    fn binary_search_empty_test() {
        let index = binary_search(&1, &vec![]);
        assert_eq!(index, None);
    }
}
