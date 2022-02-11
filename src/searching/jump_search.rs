use super::linear_search;
use std::cmp::Ordering;

/// An implementation of jump search with optimal `step`.
///
/// Jump Search is a searching algorithm for sorted arrays.
/// The basic idea is to check fewer elements (than linear search) by jumping ahead
/// by fixed steps or skipping some elements in place of searching all elements.
/// - the best/optimal block/jump size is √N
/// - Worst case time complexity: O(√N)
/// - Average case time complexity: O(√N)
/// - Best case time complexity: O(1)
/// see https://harkishen-singh.github.io/jump-search-visualisation/
///
/// # Examples
///
/// ```
/// use yumorithm::searching;
///
/// let arr = [1, 5, 7, 15, 31, 32, 45];
/// assert_eq!(searching::jump_search(&arr, &15), Some(3));
/// ```
/// ┌────────────────────────────┐
/// │ 1 12 23 23 34 45 54 76 999 │       find item = 34
/// └─┬─────▲────────────────────┘
///   │     │
///   └─────┘
///
/// ┌────────────────────────────┐
/// │ 1 12 23 23 34 45 54 76 999 │
/// └──────┬────────────▲────────┘
///        │            │
///        └────────────┘
///
///               ┌──┬──┐
///               │  │  │
///  ┌────────────▼──▼──┴─────────┐
///  │ 1 12 23 23 34 45 54 76 999 │
///  └──────┬────────────▲────────┘
///         │            │
///         └────────────┘
pub fn jump_search<T: Ord>(slice: &[T], item: &T) -> Option<usize> {
    if slice.is_empty() {
        return None;
    }

    // Finding block/jump size to be jumped
    let step = (slice.len() as f64).sqrt() as usize;

    // why needs mut? calling `nth()` multiple times doesn't rewind the iterator (see iter.nth)
    let mut iter = slice.iter();
    let mut pos: usize = 0;

    // if found larger than the value
    let mut found = false;

    // block size = (slice.len() / step)
    for i in 0..(slice.len() / step) {
        // iter.next().unwrap() = current element value
        match item.cmp(iter.next().unwrap()) {
            Ordering::Less => {
                if i == 0 {
                    // smaller than every element
                    return None;
                }
                // else
                found = true;
                break;
            }
            Ordering::Equal => return Some(i * step),
            Ordering::Greater => {
                pos = i * step;
                if i >= slice.len() {
                    // larger than every element
                    return None;
                }
            }
        }

        // Iterator::nth(...) adds 1 automatically; hence step - 1
        // additionally, Iterator::next() is invoked above; thus step - 2
        iter.nth(step - 2).unwrap();
    }

    let mut end = pos + step;
    if !found {
        // if did not found, then check the rest of the slice, since the loop
        // may not have done it; plus pos + step could be higher than len - 1
        end = slice.len();
    }

    // linear search to find the element in block (with size √N)
    linear_search(item, &slice[(pos + 1)..end]).map(|x| x + pos + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jump_empty_test() {
        assert_eq!(jump_search(&[], &4), None);
    }

    #[test]
    fn jump_not_found_test() {
        assert_eq!(jump_search(&[2, 5, 6, 11], &4), None);
        assert_eq!(jump_search(&[2, 5, 6, 11], &-1), None);
        assert_eq!(jump_search(&[2, 5, 6, 11], &110), None);
    }

    #[test]
    fn jump_search_test() {
        assert_eq!(jump_search(&[2, 5, 6, 11], &2), Some(0));
        assert_eq!(jump_search(&[2, 5, 6, 11], &5), Some(1));
        assert_eq!(jump_search(&[2, 5, 6, 11], &11), Some(3));
    }
}
