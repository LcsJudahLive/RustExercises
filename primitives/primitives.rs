use std::fmt;

fn main() {

    // Regular annotation for boolean and float variable
    let logical: bool = true;
    let float: f64 = 1.0;

    // Suffix annotation
    let integer_suffix = 5i32;

    // Default types
    let default_float = 3.0; //`f64`
    let default_int = 7; // `i32`

    // Type inference
    let mut inferred_type = 12;
    inferred_type = 444444i64; // Inferring that type is i64

    let mut mutable = 12; // Mutable `i32`
    mutable = 21;

    // Since type was inferred previosly we can't change the type to a boolean for example like:
    // mutable = true; <---- WRONG

    // We can use var shadowing to reassign a new type
    let mutable = true;

    println!("Mutable is: {}", mutable);

}