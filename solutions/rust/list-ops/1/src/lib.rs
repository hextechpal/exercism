/// Yields each item of a and then each item of b
pub fn append<I, J>(mut a: I, mut b: J) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    J: Iterator<Item = I::Item>,
{
    // this empty iterator silences a compiler complaint that
    // () doesn't implement Iterator
    std::iter::from_fn(move || match a.next() {
        Some(item) => Some(item),
        None => b.next(),
    })
}

/// Combines all items in all nested iterators inside into one flattened iterator
pub fn concat<I>(mut nested_iter: I) -> impl Iterator<Item = <I::Item as Iterator>::Item>
where
    I: Iterator,
    I::Item: Iterator,
{
    // keep the current inner iterator between calls
    let mut current: Option<I::Item> = None;
    std::iter::from_fn(move || {
        loop {
            if let Some(ref mut it) = current {
                if let Some(item) = it.next() {
                    return Some(item);
                } else {
                    // current inner iterator exhausted, drop it and try next
                    current = None;
                    continue;
                }
            }

            // try to get the next inner iterator
            match nested_iter.next() {
                Some(it) => {
                    current = Some(it);
                    continue;
                }
                None => return None,
            }
        }
    })
}

/// Returns an iterator of all items in iter for which `predicate(item)` is true
pub fn filter<I, F>(mut iter: I, predicate: F) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    F: Fn(&I::Item) -> bool,
{
    // this empty iterator silences a compiler complaint that
    // () doesn't implement Iterator
    std::iter::from_fn(move || {
        let mut ans = None;
        loop {
            match iter.next() {
                Some(item) => {
                    if (predicate)(&item) {
                        ans = Some(item);
                        break;
                    }
                }
                None => break,
            }
        }
        ans
    })
}

pub fn length<I: Iterator>(mut iter: I) -> usize {
    let mut count = 0;
    loop {
        match iter.next() {
            Some(_) => count += 1,
            None => break,
        }
    }
    count
}

/// Returns an iterator of the results of applying `function(item)` on all iter items
pub fn map<I, F, U>(mut iter: I, function: F) -> impl Iterator<Item = U>
where
    I: Iterator,
    F: Fn(I::Item) -> U,
{
    std::iter::from_fn(move || match iter.next() {
        Some(item) => Some((function)(item)),
        None => None,
    })
}

pub fn foldl<I, F, U>(mut iter: I, initial: U, function: F) -> U
where
    I: Iterator,
    F: Fn(U, I::Item) -> U,
{
    let mut init = initial;
    loop {
        match iter.next() {
            Some(item) => init = (function)(init, item),
            None => break,
        }
    }
    init
}

pub fn foldr<I, F, U>(mut iter: I, initial: U, function: F) -> U
where
    I: DoubleEndedIterator,
    F: Fn(U, I::Item) -> U,
{
    let mut init = initial;
    loop {
        match iter.next_back() {
            Some(item) => init = (function)(init, item),
            None => break,
        }
    }
    init
}

/// Returns an iterator with all the original items, but in reverse order
pub fn reverse<I: DoubleEndedIterator>(mut iter: I) -> impl Iterator<Item = I::Item> {
    // this empty iterator silences a compiler complaint that
    // () doesn't implement Iterator
    std::iter::from_fn(move || match iter.next_back() {
        Some(item) => Some(item),
        None => None,
    })
}
