// move_semantics2.rs
//
// Make the test pass by finding a way to keep both Vecs separate!
//
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand
// for a hint.



#[test]
fn test_fill_vec() {
    let vec0 = vec![22, 44, 66];

    let mut vec1 = fill_vec(vec0); // Pass in the vector by value

    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(88); // Add an element

    vec // Return the modified vector
}