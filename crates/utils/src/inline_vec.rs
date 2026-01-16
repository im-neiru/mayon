#[derive(Clone, Copy, Debug)]
pub struct InlineVec<T, const CAPACITY: usize> {
    array: [T; CAPACITY],
    length: usize,
}
