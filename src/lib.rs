#![feature(is_sorted)]
#![feature(iter_order_by)]

pub mod searching;
mod sorting;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
