use std::{boxed, mem};

#[derive(Debug)]
struct Rectangle<T, U> {
    width: T,
    height: U,
}
impl<T, U> Rectangle<T, U> {
    fn get_width(&self) -> &T {
        &self.width
    }
}
impl Rectangle<u8, u8> {
    fn get_perimeter(&self) -> u8 {
        2 * self.width + 2 * self.height
    }
}
pub fn generic_struct() {
    let r1 = Rectangle {
        width: 1,
        height: 3,
    };
    let r2 = Rectangle {
        width: 1.4,
        height: "Bags",
    };
    println!("R1: {:?}", r1);
    println!("R2: {:?}", r2);
    println!("Width of R2: {}", r2.get_width());
    println!("Perimeter R1: {}", r1.get_perimeter());
}

fn get_biggest<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b {
        return a;
    } else {
        return b;
    }
}
pub fn generic_functions() {
    println!("Biggest is: {}", get_biggest(1, 2))
}


#[derive(Debug)]
//Box<T> Generics
struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64,
}

pub fn box_generics() {
    let vehicle = Shuttle {
        name: String::from("Atlantis"),
        crew_size: 7,
        propellant: 835958.0,
    };
    println!(
        "Vehicle size on stack: {} bytes",
        mem::size_of_val(&vehicle)
    );

    let boxed_vehicle: Box<Shuttle> = Box::new(vehicle);

    println!(
        "Vehicle size on stack: {} bytes",
        mem::size_of_val(&boxed_vehicle)
    );
    println!(
        "Vehicle size on Heap: {} bytes",
        mem::size_of_val(&*boxed_vehicle)
    );
    let unboxed_vehicle: Shuttle = *boxed_vehicle;
    println!("{:?}", unboxed_vehicle);
}

fn sum_boxes<T: std::ops::Add<Output = T>>(a: Box<T>, b: Box<T>) -> Box<T> {
    return Box::new(*a + *b);
}

pub fn test_sum(){
    let one = Box::new(1);
    let two = Box::new(2);
    assert_eq!(*sum_boxes(one, two), 3);
    println!("Test Passed")

}