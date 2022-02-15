/// Insertion Sort Algorithm
/// It is a simple sorting algorithm that builds the final sorted array one at a time.
/// The List is virtually split into a sorted and an unsorted part.
/// Values from the unsorted part are picked and placed at the correct
/// position in the sorted part.
///
/// - For small set of data it is quite efficient
/// - It's stable that is it does not change the relative order of elements with equal keys
/// - Worst case performance: O(n^2)
/// - Best case performance: O(n)
///
/// ┌───────────┬───┬────────────┐
/// │  sorted   │ x │  unsorted  │
/// └───────────┴───┴────────────┘
///
/// ┌──────────────┐
/// │ 12 11 13 5 6 │
/// └─┬────────────┘
///   │  unsorted
///   x
///
///   ┌───┐
/// ┌─▼───▼────────┐
/// │ 12 11 13 5 6 │  from x position to start, search and put 11 in the right position
/// └────┬─────────┘
///      │  unsorted
///      x
///
/// ┌──────────────┐
/// │ 11 12 13 5 6 │  13 will remain at  its positions because all element are smaller then 13
/// └───────┬──────┘
///  sorted │ unsorted
///         x
///
///     ┌─────┐
/// ┌───▼──▼──▼────┐
/// │ 5 11 12 13 6 │
/// └──────────┬───┘
///    sorted  │
///            x
///
///     ┌───────┐
/// ┌───▼──▼─▼──▼──┐
/// │ 5 6 11 12 13 │
/// └───────────┬──┘
///             │
///             x
///
pub fn insertion_sort<T: PartialOrd + Copy>(arr: &mut [T]) {
    if arr.len() == 0 {
        return;
    }
    for i in 1..arr.len() {
        let cur = arr[i]; // Copy trait for T
        let mut j = i - 1;

        while arr[j] > cur {
            arr.swap(j + 1, j);
            if j == 0 {
                // Array index operator and swap accpets parameters with usize type
                // Thefore to prevent underflow we need to break
                break;
            }
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_sort_empty_test() {
        let mut empty: Vec<i32> = Vec::new();
        insertion_sort(&mut empty);
        assert_eq!(empty, Vec::new());
    }

    #[test]
    fn insert_sort_test() {
        struct Tests {
            cases: Vec<Vec<i32>>,
        }
        let mut tests = Tests {
            cases: vec![
                vec![46, 50, 41, -3, 2, 0],
                vec![1, 2, 3],
                vec![3, 2, 1],
                vec![1, 2, 3, 5, 4],
            ],
        };

        for t in tests.cases.iter_mut() {
            insertion_sort(t);
            assert_eq!(true, t.is_sorted());
        }
    }
}
