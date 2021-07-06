fn main() {
    // Mutable borrow 
    let mut language = "Rust";
    let other_language = &mut language;

    // Comparing reference value on if condition
    if *other_language == "Rust" {
        println!("You're just learning Rust");
    }

    // Changing reference value, it'll also change language's var value
    *other_language = "Golang";

    // Now if condition is false 'cause value is Golang
    if *other_language == "Rust" {
        println!("You're just learning Rust");
    } else if *other_language == "Golang" {
        println!("You're now learning Golang");
    }

    let course = ("Rust", "beginner","course");
    // pattern matches with the scrutinee expression
    // Partial match
    if let ("Rust", _c, _d ) = course {
        println!("Wrote all values in pattern to be matched with the scrutinee expression");
    } else {
        // do not execute this block
        println!("Value unmatched");
    }

    // Pattern not defined
    if let _ = 10 {
        println!("irrefutable if-let pattern is always executed");
    }

    // MATCH == SWITCH CASE
    let course = "Rust";
    // return value of match expression in a variable
    let found_course = match course {
        "Rust" => "Rust",
        "Java" => "Java",
        "C++" => "C Plus Plus",
        "C#" => "C Sharp",
        _ => "Unknown Language"
    };
    println!("Course name : {}",found_course);


}