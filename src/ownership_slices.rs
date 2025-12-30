pub fn ownership_slices_main() {
    let s = String::from("Hello, world!");
    let first_word = first_word(&s);
    println!("The first word in {s} is {first_word}");

    // s.clear();
    // println!("If you keep {first_word} alive then the compiler won't let you modify 's' as you have an immutable reference to it.")
}

fn first_word(str: &str) -> &str {
    let bytes = str.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &str[0..i];
        }
    }

    return &str
}

// Notes:
// bytes.iter().enumerate() -> iter() returns each element in a collection, .enumerate() wraps the result, and returns each element in a tuple with it's index.
// The type &str is a string slice. It defines a reference to a string.