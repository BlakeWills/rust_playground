pub fn pattern_matching_main() {
    
    let library: Vec<Book> = vec![
        Book { title: String::from("Lord of the Rings"), is_available: true },
        Book { title: String::from("Harry Potter"), is_available: false },
        Book { title: String::from("Maze Runner"), is_available: true }
    ];
    
    // Pattern match
    match find_book("Harry Potter", &library) {
        None => println!("Could not find book!"),
        Some(b) => println!("Found book! is_available: {0}", b.is_available)
    };

    // nested match
    match find_book("Harry Potter", &library) {
        None => println!("Could not find book!"),
        Some(b) => match b.is_available {
            true => println!("{0} is available!", b.title),
            false => println!("{0} is not available!", b.title)
        }
    };

    // flattened nested match using resource guards (with some different syntax for printing)
    match find_book("Harry Potter", &library) {
        None => println!("Could not find book!"),
        Some(b) if b.is_available => println!("{} is available!", b.title),
        Some(b) => println!("{title} is not available!", title = b.title),
    };

    //// Explicit .unwrap() - panics when unwrapping a None
    //let not_found = find_book("Hunger Games", &library);
    //let this_will_crash = not_found.unwrap();

    // if we only want to print out when we've found the book, we can use if let instead:
    if let Some(found) = find_book("Maze Runner", &library) {
        println!("We found {0} and is_available: {1}", found.title, found.is_available);
    }
}

struct Book {
    title: String,
    is_available: bool
}

fn find_book<'a>(title: &str, books: &'a Vec<Book>) -> Option<&'a Book> {
    books.iter().find(|b| b.title == title)
} 