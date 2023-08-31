//From kmdreko, https://stackoverflow.com/questions/69764050/how-to-get-the-indices-that-would-sort-a-vec
pub fn argsort<T: Ord>(data: &[T]) -> Vec<usize> {
    let mut indices = (0..data.len()).collect::<Vec<_>>();
    indices.sort_by_key(|&i| &data[i]);
    indices
}