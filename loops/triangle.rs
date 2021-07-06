fn test(n:i32) {
    let mut columns = 0;

    for i in 0..n {
        columns += 1;
        for j in 0..columns {
            print!("{}", '&');
        }
        print!("\n");
    }
}

fn main(){
    test(20);
}