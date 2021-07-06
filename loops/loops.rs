fn main() {
    //iterate over range
    for i in 0..5 {
        println!("{}", i);
    }
    
    //Iterate printing count and number of times loop has executed, "count" is the enumerate value
    for (count, variable) in (7..10).enumerate() {
        println!("count = {}, variable = {}", count, variable);
    }

    // WHILES
    // Given a range of integers stop the loop printing first multiple of 7
    let mut found = false;
    let mut index = 0;
    let arr:[i32; 10] = [12342, 1205, 5403, 103, 435, 10, 43, 89, 24, 222];

    while !found {
        // Adding as first statement to avoid array index out of bounds
        if index >= arr.len() {
            println!("Didn't find any multiple of 7");
            break;
        } else if arr[index] % 7 != 0 {
            println!("{} not multiple of 7", arr[index]);
            index += 1
        } else if arr[index] % 7 == 0 {
            println!("Found It! {} is multiple of 7!", arr[index]);
            found = true;
        }
    }

    // THERE'S ALSO `loop` that runs indefinetely (you can break it with `break`)

}