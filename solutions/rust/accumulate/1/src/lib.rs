/// What should the type of function be?
pub fn map<T, U, V>(input: Vec<T>, mut function: U) -> Vec<V>
where
    U: FnMut(T) -> V,
{
    let mut res: Vec<V> = Vec::with_capacity(input.len());
    for el in input {
        res.push((function)(el).into());
    }
    res
}
