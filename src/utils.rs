pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

//pub fn window<I: Iterator<Item = U>, U>(mut iterator: I, n: usize) -> Option<Vec<U>> {
//    let mut window = vec![];
//    for _ in 0..n {
//        let elt = iterator.next()?;
//        window.push(elt);
//    }
//    Some(window)
//}//

//pub fn next_chunk_to_vec<I: Iterator<Item = U>, U>(mut iterator: I, size: usize) -> Option<Vec<U>> {
//    let mut res = Vec::with_capacity(size);
//    for _ in 0..size {
//        res.push(iterator.next()?)
//    }
//    Some(res)
//}
