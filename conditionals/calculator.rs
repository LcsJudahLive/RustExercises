fn calc(_a:i32, _b:i32, _operator:char) {
    
    match _operator {
        '+' => { 
            println!("{}", _a + _b);
        },
        '-' => {
            println!("{}", _a - _b);
        },
        'x' => {
            println!("{}", _a * _b);
        },
        '/' => {
            if _b != 0 {
                println!("{}", _a / _b);
            } else {
                println!("Division by 0 is undefined");
            }
        },
        '%' => {
            println!("{}", _a % _b);
        },
        _   => println!("Invalid operator")
    };
}

fn main(){
    calc(3, 0, '/');
    calc(3, 1, '+');
    calc(3, 0, '&')
}