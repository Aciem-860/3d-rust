pub struct Tuple<T, U> {
    pub first: T,
    pub second: U,
}

impl<T, U> Tuple<T, U> {
    pub fn new(first: T, second: U) -> Tuple<T, U> {
        Tuple { first, second }
    }
}
