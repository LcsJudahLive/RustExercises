fn main() {
    
    // Unsigned Addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Signed subtraction
    println!("1 - 2 = {}", 1i32 - 2);
    //println!("1 - 2 = {}", 1u32 - 2); <--- IT DOESN'T WORK
    
    // Playing around with booleans
    println!("true AND false: {}", true && false);
    println!("true OR false: {}", true || false);
    println!("NOT true: {}", !true);

    // Bit operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    //Using underscores for readability
    println!("One million with underscores: {}", 1_000_000u32);
}