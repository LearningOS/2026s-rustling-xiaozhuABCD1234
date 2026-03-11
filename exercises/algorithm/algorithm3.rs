/*
    sort
    This problem requires you to implement a sorting algorithm
    you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: Ord>(array: &mut [T]) {
    //TODO
    if array.len() > 1 {
        let high = array.len() - 1;
        quick_sort(array, 0, high);
    }
}

fn quick_sort<T: Ord>(array: &mut [T], low: usize, high: usize) {
    if low >= high {
        return;
    }
    let p = partition(array, low, high);
    if p > 0 {
        quick_sort(array, low, p - 1);
    }
    quick_sort(array, p + 1, high);
}

fn partition<T: Ord>(array: &mut [T], left: usize, right: usize) -> usize {
    let (mut i, mut j) = (left, right);
    while i < j {
        while i < j && array[left] <= array[j] {
            j -= 1;
        }
        while i < j && array[left] >= array[i] {
            i += 1;
        }
        array.swap(i, j);
    }
    array.swap(i, left);
    return i;
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
