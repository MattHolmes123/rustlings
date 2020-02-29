// strings2.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings2` for hints :)

fn main() {
    // Commented out so play with String methods.
    // let word = String::from("green"); // Try not changing this line :)


    // Create a growable mutable String variable
    let mut word = String::new();
    // use method to add a string slice
    word.push_str("gr");

    // using + syntax to concatinate the string;
    word += "ee";

    // using push `char` method.
    word.push('n');

    if is_a_color_word(&word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}

fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}
