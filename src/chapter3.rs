fn say_hello() {
    println!("Say hello");
    say_a_number(42);
    say_the_sum(1, 2)
}

fn say_a_number(number: i32) {
    println!("Number: {}", number);
}

fn say_the_sum(a: u8, b: u8) {
    println!("The sum is: {}", a + b)
}
fn squared(x: i32) -> i32 {
    x * x
}
fn late_squared(x: i32) -> i32 {
    let square = x * x;
    println!("Squared is {}", square);
    return square;
}
fn multiple_returns(x: i32) ->(i32, i32) {
    let squared = x * x;
    let cubed = x * x * x;
    println!("Function Complete");
    return (squared, cubed)
}

fn celsius_to_farenheit(celsius: f64) -> f64 {
    let farenheit: f64;
    farenheit = (1.8 * celsius) + 32.0;
    return farenheit
}