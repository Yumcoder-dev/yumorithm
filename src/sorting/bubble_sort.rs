/// It is a comparison-based algorithm in which each pair of adjacent elements is compared
/// and the elements are swapped if they are not in order.
/// After each iteration, at least one value moves at the end
///
/// ┌────────────────┐
/// │ 14 33 27 35 10 │   bubble sort first iteration
/// └─┬───▲──────────┘
///   │   │
///   └───┘
///
/// ┌────────────────┐
/// │ 14 33 27 35 10 │   27 < 33, so exchange them
/// └────┬──▲────────┘
///      │  │
///      └──┘
///
/// ┌────────────────┐
/// │ 14 27 33 35 10 │   33 < 35
/// └───────┬──▲─────┘
///         │  │
///         └──┘
///
///  ┌────────────────┐
///  │ 14 27 33 10 35 │  35 > 10, swapped them
///  └──────────┬──▲──┘
///             │  │
///             └──┘
/// After the first iteration the largest number be at last index of the array
///
pub fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    for i in 0..arr.len() {
        let mut swapped = false;
        for j in 0..arr.len() - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bubble_sort_empty_test() {
        let mut empty: Vec<i32> = Vec::new();
        bubble_sort(&mut empty);
        assert_eq!(empty, Vec::new());
    }

    #[test]
    fn bubble_sort_test() {
        let mut tests = vec![
            vec![46, 50, 41, -3, 2, 0],
            vec![1, 2, 3],
            vec![3, 2, 1],
            vec![1, 2, 3, 5, 4],
        ];

        for t in tests.iter_mut() {
            bubble_sort(t);
            assert_eq!(true, t.is_sorted());
        }
    }
}
