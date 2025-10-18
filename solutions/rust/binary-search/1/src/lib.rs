pub fn find<T, A>(a: A, key: T) -> Option<usize>
where
    T: Ord,
    A: AsRef<[T]>,
{
    
    let array = a.as_ref();

    if array.is_empty() {
        return None;
    }

    let mut l = 0;
    let mut r = array.len();
    while l < r {
        let mid: usize = l + (r - l) / 2;

        match array[mid].cmp(&key) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Less => l = mid+1,
            std::cmp::Ordering::Greater => r=mid     
        }
    }

    None
}
