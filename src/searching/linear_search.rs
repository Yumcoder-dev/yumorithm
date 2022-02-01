
// Problem: a linear search or sequential search is a method for finding 
// an element within a list. It sequentially checks each element of 
// the list until a match is found or the whole list has been searched.
//
// item: references to search item
// arr: references to array that search in it
//
// All inputs should be T that implemented PartialEq trait for
// equality comparisons (`x == y` and `x != y`)
pub fn linear_search<T: PartialEq>(item: &T, arr: &[T]) -> Option<usize> {
    // create an iterator that returns (index, data)
    // 
    // # Examples
    //
    // ```
    // let a = ['a', 'b', 'c'];
    //
    // let mut iter = a.iter().enumerate();
    //
    // assert_eq!(iter.next(), Some((0, &'a')));
    // assert_eq!(iter.next(), Some((1, &'b')));
    // assert_eq!(iter.next(), Some((2, &'c')));
    // assert_eq!(iter.next(), None);
    // ```
    for (i , data) in arr.iter().enumerate() {
        if data == item {
            return Some(i)
        }
    }
    None
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_strings() {
        let index = linear_search(&"a", &vec!["a", "b", "c", "d"]);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn search_ints() {
       struct Tests {
            ds : Vec<i32>,
            cases : Vec<(i32, Option<usize>)>
        }
        let tests = Tests {
            ds:vec![1, 2, 3, 4],
            cases: vec![
                (1, Some(0)), 
                (2, Some(1)), 
                (3, Some(2)), 
                (4, Some(3)), 
            ]
        };

        for test in tests.cases.iter() {
            let index = linear_search(&test.0, &tests.ds);
            assert_eq!(index, test.1);
        }
    }

    #[test]
    fn not_found() {
        let index = linear_search(&5, &vec![1, 2, 3, 4]);
        assert_eq!(index, None);
    }

    #[test]
    fn empty() {
        let index = linear_search(&1, &vec![]);
        assert_eq!(index, None);
    }
}
