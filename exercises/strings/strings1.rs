// strings1.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings1` for hints ;)

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}

fn current_favorite_color() -> String {
    // just to remind me that &str is a string slice.
    let blue_string_slice = "blue";

    blue_string_slice.to_string()
}
