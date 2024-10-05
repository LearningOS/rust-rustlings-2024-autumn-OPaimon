/*
    sort
    This problem requires you to implement a sorting algorithm
    you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn merge<T: Ord + Copy>(array: &mut [T], mid: usize) {
    let left = array[..mid].to_vec();
    let right = array[mid..].to_vec();
    let mut left_iter = left.into_iter();
    let mut right_iter = right.into_iter();

    let mut left_peek = left_iter.next();
    let mut right_peek = right_iter.next();

    for elem in array.iter_mut() {
        *elem = match (left_peek.clone(), right_peek.clone()) {
            (Some(l), Some(r)) => {
                if l <= r {
                    left_peek = left_iter.next();
                    l
                } else {
                    right_peek = right_iter.next();
                    r
                }
            }
            (Some(l), None) => {
                left_peek = left_iter.next();
                l
            }
            (None, Some(r)) => {
                right_peek = right_iter.next();
                r
            }
            (None, None) => unreachable!(),
        };
    }
}

fn sort<T: Ord + Copy>(array: &mut [T]) {
    //TODO
    // array.sort();
    if array.len() < 2 {
        return;
    }
    let mid = array.len() / 2;
    let (left, right) = array.split_at_mut(mid);
    sort(left);
    sort(right);
    merge(array, mid)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
    #[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
    #[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}
