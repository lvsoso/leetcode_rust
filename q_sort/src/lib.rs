fn pivot<T: Ord>(v: &mut [T]) -> usize {
    let mut p = 0;
    for i in 1..v.len(){
        if v[i] < v[p] {
            // p+1 <> i
            v.swap(p+1, i);
            // p <> p+1
            v.swap(p, p+1);

            // p => p+1
            p += 1;
        }
    }
    p
}

fn quick_sort<T: Ord + std::fmt::Debug>(v: &mut [T]){
    if v.len() <= 1 {
        return ;
    }
    let p = pivot(v);
    println!("{:?}", v);
    let (a , b) = v.split_at_mut(p);
    quick_sort(a);
    quick_sort(&mut b[1..]);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pivot() {
        let mut v = vec![4, 6, 1, 56, 23, 1, 2];
        let p = pivot(&mut v);
        for x in 0..v.len(){
            assert!((v[x] < v[p]) == (x < p));
        }
    }

    #[test]
    fn test_quick_sort(){
        let mut v = vec![4, 8,2, 35, 7, 43];
        quick_sort(&mut v);
        assert_eq!(v, vec![2, 4, 7, 8, 35, 43]);
    }
}
