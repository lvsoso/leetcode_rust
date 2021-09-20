
fn merge_sort<T>(mut v: Vec<T>) -> Vec<T>
    where T: Ord + std::fmt::Debug {
    if v.len() <= 1{
        return v;
    }

    let mut res = Vec::with_capacity(v.len());

    let b = v.split_off(v.len()/2);
    // 1. sort the left half
    let a = merge_sort(v);
    // 2. sort the right halr
    let b = merge_sort(b);
    // 3. bring the sorted halfs together
    let mut a_it = a.into_iter();
    let mut b_it = b.into_iter();
    let mut a_peek = a_it.next();
    let mut b_peek = b_it.next();
    loop {
        match a_peek {
            Some(ref a_val) => match b_peek {
                Some(ref b_val) => {
                    if b_val < a_val {
                        res.push(b_peek.take().unwrap());
                        b_peek = b_it.next();
                    }else{
                        res.push(a_peek.take().unwrap());
                        a_peek = a_it.next();
                    }
                }
                None => {
                        res.push(a_peek.take().unwrap());
                        res.extend(a_it);
                        return res;
                }
            }
            None => {
                if let Some(b_val) = b_peek {
                    res.push(b_val);
                }
                res.extend(b_it);
                return res;
            }
        }
    }


}

#[cfg(test)]
mod tests {
    use super::merge_sort;


    #[test]
    fn test_merge_sort() {
        let v = vec![5, 67, 2, 4, 56, 78, 99];
        let v = merge_sort(v);
        assert_eq!(v, vec![2, 4, 5, 56, 67, 78, 99])
    }
}
