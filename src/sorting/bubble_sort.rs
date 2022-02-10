pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        for j in 0..arr.len() - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bubble_sort_test() {
        let mut arr = vec![46, 50, 41, -3, 2, 0];
        bubble_sort(&mut arr);
        for i in 0..arr.len() - 1 {
            assert!(arr[i] <= arr[i + 1]);
        }
    }
}
