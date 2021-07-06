fn main() {
    /// - Shared Borrowing: Piece of data shared by single or multiple variable, can't be altered
    /// - Mutable Borrowing: Piece of data shared by single variable that can alter its value (not accessible for other variables)

    let x = 10;
    let mut y = 13;

    // Shared Borrow
    let a = &x;
    println!("Value of a:{}", a); //IT SHOULD PRINT 10
    println!("Value of x:{}", x); //IT SHOULD PRINT 10

    // Mutable Borrow
    let b = &mut y;
    println!("Value of b:{}", b); //IT SHOULD PRINT 13

    *b = 11; // Deferencing operator, it will update b and y values, since it's mutuably borrowed
    println!("Value of b:{}", b); // IT SHOULD PRINT 11
    println!("Value of y:{}", y); // IT SHOULD PRINT 11


}