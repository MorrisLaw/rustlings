// vecs1.rs
//
// Your task is to create a `Vec` which holds the exact same elements as in the
// array `a`.
//
// Make me compile and pass the test!
//
// Execute `rustlings hint vecs1` or use the `hint` watch subcommand for a hint.
// IM NOT DONE
fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // a plain array
    // THIS IS ONE WAY TO DO IT. Loop through and push each value.
    //let mut v: Vec<i32> = Vec::new();
    //for i in a {
    //    v.push(i)
    //}
    // THIS IS ANOTHER WAY. Use a slice to convert to vec.
    let v = a.to_vec();
    (a, v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, v[..]);
    }
}
