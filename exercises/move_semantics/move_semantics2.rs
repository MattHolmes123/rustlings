// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)

fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(&vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    // Example of passing a mutable reference to the function.
    let mut vec2 = Vec::new();
    fill_vec_by_mutable_reference(&mut vec2);
    vec2.push(110);
    println!("{} has length {} content `{:?}`", "vec2", vec2.len(), vec2);
}

fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
    let mut vec = vec.to_vec();

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}

fn fill_vec_by_mutable_reference(vec: &mut Vec<i32>) -> () {
    vec.push(22);
    vec.push(44);
    vec.push(66);
}