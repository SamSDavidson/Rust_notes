fn math() -> f64 {
    let x = 10;
    let y = 22.345;
    let z = x as f64 + y;
    return z;
}

fn manage_print() {
    let x = 10;
    let y = x as f64 + 22.345999;

    let _ts1 = ("y is {:.3}", y);
    let _ts2 = ("y is {:8.3}", y);
    let _ts3 = ("y is {:08.3}", y);
    let _ts4 = ("x is {}\ny is {}", x, y);
    let _ts5 = ("x is {0}\ny is {1}\n In case you forget, x is {0}, ", x, y);
    let _ts6 = "x is {x}\ny is {y}\n In case you forget, x is {x}";
}

fn bit_ops() {
    let value = 0b1111_0101;
    println!("The value is: {value}");

    let mut u8_value = 0b1111_0101u8;
    println!("U8 Value is: {u8_value}");

    println!("The binary bits are {:08b}", u8_value);

    // Bitwise Operators

    //NOT
    u8_value = !u8_value;
    println!("The NOT Binary bits are: {:08b}", u8_value);

    //AND
    u8_value = u8_value & 0b1111_0111;
    println!("The AND Binary bits are: {:08b}", u8_value);

    //AND to check for a bit value
    u8_value = u8_value & 0b1111_0111;
    println!("Bit 6 is: {}", u8_value & 0b0100_0000);

    //OR
    u8_value = u8_value | 0b0100_0000;
    println!("Bit 6 is changed to 1: {:08b}", u8_value);

    //XOR
    u8_value = u8_value ^ 0b0100_0000;
    println!("Bit 6 XOR is: {:08b}", u8_value);

    //SHIFT
    u8_value = u8_value << 4; //4 is the value to shift by << means left
    println!("Bit 6 left 4 shift is: {:08b}", u8_value);
    u8_value = u8_value >> 2; // shift right by two
    println!("Bit 6 right 2 shift is: {:08b}", u8_value);
}

fn boolean_types() {
    let a = true;
    let b = false;
    println!("a is {a} and b is {b}"); // Individual values
    println!("Not a is {}", !a); // Reverse value
    println!("a AND b is {}", a & b); //Both values
    println!("a OR b is {}", a | b); //Either or
    println!("a XOR b is {}", a ^ b); //One but not the other

    // More complex options
    let c = (a ^ b) | (a & b);
    println!("{}", c); //Must have a string literal for println!

    //Short Circuiting uses && and ||
    let c = (a ^ b) || (a & b); //skips right side as it determiens answer from first
    println!("Short circuit c is {}", c);

    //Comparison operators are standard < > == !=...

    //Can be used for values other than numbers
    let d = true;
    let e = false;

    println!("d is {} and e is {}", d, e);
    println!("d == e = {}", d == e);
    println!("d != e = {}", d != e);
    println!("d > e = {}", d > e);
    println!("d < e = {}", d < e);
}

fn char_types() {
    // Single strings denotes 'char'
    let letter = 'a';
    let number = '1';

    let finger = '\u{261D}';

    println!("{}\n{}\n{}", letter, number, finger);
}

fn challenge_section() {
    let avg: f64 = find_average(13, 2.3, 120.0);

    assert_eq!(avg, 45.1); //checks if values are equal, panics if not
    println!("Passed!");
}
fn find_average(a: i32, b: f64, c: f64) -> f64 {
    let total: f64 = a as f64 + b as f64 + c as f64;
    let average: f64 = total / 3.0;
    return average;
}
