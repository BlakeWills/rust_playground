// try_into
// unwrap (result) and pattern matching
// traits

mod ownership;
mod ownership_slices;
mod pattern_matching;
mod result;
mod traits;
mod enumerations;

fn main() {
    println!("Hello, world!");
    ownership::ownership_main();
    ownership_slices::ownership_slices_main();

    pattern_matching::pattern_matching_main();

    result::result_main();

    traits::traits_main();

    enumerations::enumerations_main();
}