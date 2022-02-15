/// It is a comparison-based algorithm in which sorts an array by repeatedly finding
/// the minimum element (considering ascending order) from unsorted part and putting
/// it at the right place.
/// ┌───────────┬───┬────────────┐
/// │  sorted   │ x │  unsorted  │
/// └───────────┴───┴────────────┘
///
/// ┌──────────────┐
/// │ 12 11 13 5 6 │ find the min 1..4
/// └─┬────────────┘
///   │  unsorted
///   x
///
///   ┌───────┐
/// ┌─▼───────▼────┐
/// │ 5 11 13 12 6 │  swap(5, 12)
/// └────┬─────────┘
///      │  unsorted
///      x
/// ┌──────────────┐
/// │ 5 6  13 12 11│  swap(6, 11)
/// └───────┬──────┘
///  sorted │ unsorted
///         x
///
/// ┌──────────────┐
/// │ 5 6 11 12 13 │ swap(11, 13)
/// └─────────┬────┘
///    sorted │
///           x
///
/// ┌──────────────┐
/// │ 5 6 11 12 13 │
/// └───────────┬──┘
///             │
///             x
pub fn selection_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        let mut kth_smallest_index = i;
        for j in (i + 1)..len {
            if arr[j] < arr[kth_smallest_index] {
                kth_smallest_index = j
            }
            arr.swap(i, kth_smallest_index);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selection_sort_empty_test() {
        let mut empty: Vec<i32> = Vec::new();
        selection_sort(&mut empty);
        assert_eq!(empty, Vec::new());
    }

    #[test]
    fn selection_sort_test() {
        let mut tests = vec![
            vec![46, 50, 41, -3, 2, 0],
            vec![1, 2, 3],
            vec![3, 2, 1],
            vec![1, 2, 3, 5, 4],
        ];

        for t in tests.iter_mut() {
            selection_sort(t);
            assert_eq!(true, t.is_sorted());
        }
    }
}
