// iterators2.rs
// In this module, you'll learn some of unique advantages that iterators can offer
// Step 1. Complete the `capitalize_first` function to pass the first two cases
// Step 2. Apply the `capitalize_first` function to a vector of strings, ensuring that it returns a vector of strings as well
// Step 3. Apply the `capitalize_first` function again to a list, but try and ensure it returns a single string
// As always, there are hints if you execute `rustlings hint iterators2`!

pub fn capitalize_first(input: &str) -> String {
    // c is an iter
    let mut c = input.chars();

    // c.next consumes the first key.
    match c.next() {
        None => String::new(),

        // convert the first char to uppercase, collect as a String and finally
        // add the rest of c (all but the first char).
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
    }
}


// I think by changing the return type the collect knows what to do.
pub fn capitalize_words(input: Vec<&str>) -> Vec<String> {
    input.iter()
        .map(|x| capitalize_first(x))
        .collect()
}

// I think by changing the return type the collect knows what to do.
pub fn capitalize_words_as_string(input: Vec<&str>) -> String {
    input.iter()
        .map(|x| capitalize_first(x))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Step 1.
    // Tests that verify your `capitalize_first` function implementation
    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    // Step 2.
    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        let capitalized_words: Vec<String> = capitalize_words(words);
        assert_eq!(capitalized_words, ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        let capitalized_words = capitalize_words_as_string(words);
        assert_eq!(capitalized_words, "Hello World");
    }


    #[test]
    fn test_iterate_no_function() {
        let words = vec!["hello", "world"];

        let capitalized_words = words.iter()
            .map(|x| capitalize_first(x))
            .collect::<Vec<String>>();

        assert_eq!(capitalized_words, ["Hello", "World"]);

        let words = vec!["hello", " ", "world"];

        let capitalized_words = words.iter()
            .map(|x| capitalize_first(x))
            .collect::<String>();

        assert_eq!(capitalized_words, "Hello World");
    }
}
