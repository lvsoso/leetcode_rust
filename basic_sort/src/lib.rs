fn bubble_sort<T: Ord + std::fmt::Debug>(arr: &mut [T]) {
    for i in 0..arr.len() {
        println!("{:?}", arr);
        for j in 0..arr.len()- 1 - i {
            if arr[j] > arr[j+1] {
                arr.swap(j, j + 1);
            }
        }
    }
}


fn selection_sort<T: Ord + std::fmt::Debug>(arr: &mut [T]) {
    let len = arr.len();
    for left_index in 0..len {
        let mut smallest_index = left_index;
        for right_index in (left_index + 1)..len {
            if arr[right_index] < arr[smallest_index] {
                smallest_index = right_index;
            }
        }
        arr.swap(smallest_index, left_index);
    }
}


fn insertion_sort<T>(arr: &[T]) -> Vec<T>
where 
    T: Ord + Clone + std::fmt::Debug {
    let mut res: Vec<T> = Vec::with_capacity(arr.len());
    for item in arr.iter().cloned() {
        let n_len = res.len();
        for i in 0..=n_len{
            if i == n_len || res[i] > item {
                res.insert(i, item);
                break;
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bubble() {
        let mut vec1 = vec![4, 5, 6, 1, 3, 2];
        bubble_sort(&mut vec1);
        for i in 0..vec1.len() - 1 {
            assert!(vec1[i] <= vec1[i+1]);
        }
    }

    #[test]
    fn seletion(){
        let mut vec1 = vec![4, 5, 6, 1, 3, 2];
        selection_sort(&mut vec1);
        for i in 0..vec1.len() - 1 {
            assert!(vec1[i] <= vec1[i+1]);
        }
    }
    #[test]
    fn insetion(){
        let mut vec1 = vec![4, 5, 6, 1, 3, 2];
        let vec2 = insertion_sort(&mut vec1);
        for i in 0..vec2.len() - 1 {
            assert!(vec2[i] <= vec2[i+1]);
        }
    }
}
