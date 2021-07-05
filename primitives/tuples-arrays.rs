use std::fmt;
use std::mem;

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;

    (boolean, integer)
}

// Use derive to make struct Debug printable
// 2x2 Matrix
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn transpose(matrix: Matrix) -> Matrix {
    let transposed = Matrix(matrix.0, matrix.2, matrix.1, matrix.3);

    transposed
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {})\n({} {})", self.0, self.1, self.2, self.3)
    }
}

// Declaring slice
fn analyze_slice(slice: &[i32]) {
    println!("first element of slice {}", slice[0]);
    println!("slice length {}", slice.len());
}


fn main() {
    let long_tuple = (1u8, 2u16, 3u32, 4u64, 
                    -1i8, -2i16, -3i32, -4i64,
                    0.1f32, 0.2f64, 'a', true);

    // Print from tuple
    println!("Tuple 2nd value: {}", long_tuple.1);

    // Tuple of tuples
    let tuple_of_tuples = ((1u8, 2u16), (4u64, -1i8), -2i16);
    println!("Tuple of tuples: {:?}", tuple_of_tuples);

    //Reverse pair
    let pair = (1, true);
    println!("pair is {:?}", reverse(pair));

    println!("That's a tuple: {:?}", (5u32,));
    println!("That's an integer: {:?}", (5u32));

    // Destructuring tuples, binding to vars
    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Matrix: \n{}", matrix);
    //transpose matrix
    println!("Tranposed Matrix: \n{}", transpose(matrix));

    // ARRAYS AND SLICES
    // fixed size
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 500] = [133; 500];
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    // ARRAY LENGTH
    println!("Array xs length is {}", xs.len());
    
    // Length in memory
    println!("array xs occupies {} bytes", mem::size_of_val(&xs));
    // Borrowing array as slice
    analyze_slice(&xs);

    //Slicing as a section of array
    analyze_slice(&ys[1 .. 4]);

}
