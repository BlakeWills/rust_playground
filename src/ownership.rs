// https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

pub fn ownership_main() {
    let s = String::from("this is a string");

    // This function takes ownership of 's' so it cannot be used after the call
    let _size = calculate_len_no_refs(s);
    // println!("The size of '{s}' is {size}"); // Cannot do this because 's' has been moved into calculate_len_no_refs

    // You could transfer ownership back to the caller like so:
    let s2 = String::from("this is a weird way of doing it");
    let (s2_again, s2_size) = calculate_len_and_return_back(s2);
    println!("The size of '{s2_again}' is {s2_size}"); 
    // println!("The size of '{s2}' is {s2_size}");  // This wouldn't work as 's2' has been borrowed.

    // The best way of letting the caller retain ownership of the value is to pass a reference.
    let s3 = String::from("this is another string");
    let size3 = calculate_len_ref(&s3);
    println!("The size of '{s3}' is {size3}");

    // Mutable references
    let mut mut_s = String::from("foo");
    println!("mut_s is '{mut_s}'");
    let mut_s_size = mutate_and_calc_len(&mut mut_s);
    println!("The size of '{mut_s}' is {mut_s_size}");
}

fn calculate_len_no_refs(s: String) -> usize {
    s.len()
}

fn calculate_len_and_return_back(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}

fn calculate_len_ref(s: &String) -> usize {
    return s.len()
}

//// This function attempts to create a dangling reference by creating a variable, 's', and then returning a pointer to it.
//// This is not allowed because 's' goes out of scope, and so is cleaned up.
// fn cannot_create_dangling_ref() -> &String {
//     let s = String::from("my string");
//     return &s;
// }

fn mutate_and_calc_len(s: &mut String) -> usize {
    s.push_str("bar");
    s.len()
}