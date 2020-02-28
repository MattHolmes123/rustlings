// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is. 
// Execute `rustlings hint primitive_types3` for hints!

fn main() {

    // This creates an array of 101 elements each with the value of 42
    let a = [42; 101];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
