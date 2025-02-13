fn main() {
    let mut v = vec![1, 2, 3];
    // Instead of using raw pointers, directly modify the vector
    v[0] = 10;
    println!( "{:?}", v);
}

//Alternatively, using slices is also safe and efficient
fn main() {
    let mut v = vec![1, 2, 3];
    let slice = &mut v[..];
    slice[0] = 10;
    println!( "{:?}", v);
} 