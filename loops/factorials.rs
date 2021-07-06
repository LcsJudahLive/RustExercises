fn factorial(n:i32) {
    let mut fac = n;
    let mut index = n;
    if fac < 0 {
        fac = 0;
    } else if fac == 0 {
        fac = 1;
    } else {
        while (index - 1) >= 1 {
            fac *= index - 1;
            index -= 1;
        }
    }
    println!("{}", fac);
}

fn main() {
    factorial(3);
}