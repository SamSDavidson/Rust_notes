//Enums
#[derive(Debug)]
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

pub fn define_enum() {
    let my_shape = Shape::Rectangle(1.2, 3.4);
    println!("My shape is {:?}", my_shape);
}

pub fn match_operator() {
    let my_shape = Shape::Rectangle(1.2, 3.4);
    let my_number = 1u8;
    match my_shape {
        Shape::Circle(r) => println!("Circle with radius {}", r),
        Shape::Rectangle(w, h) => {
            println!("Rectangle with width {} and height {}", w, h)
        }
        Shape::Triangle(a, b, c) => println!("Triangle with sides {}, {} and {}", a, b, c),
    }

    let result = match my_number {
        0 => "zero",
        1 => "one",
        2 => "two",
        3 => "three",
        _ => {
            println!("{} did not match", my_number);
            "something else"
        }
    };
    println!("Match is {}", result);
}

impl Shape{
    fn get_perimeter(&self) -> f64{
        match *self{
            Shape::Circle(r) => r * 2.0 * std::f64::consts::PI,
            Shape::Rectangle(w, h) => (2.0 * w) + (2.0 * h),
            Shape::Triangle(a, b, c) => a + b + c,
        }
    }
}

pub fn enum_methods(){
    let my_shape = Shape::Rectangle(1.2, 3.4);
    println!("My shape is {:?}", my_shape);

    let perimeter = my_shape.get_perimeter();
    println!("Perimeter is {}", perimeter);
}

pub fn option_enum(){
    let countdown = [5, 4, 3, 2, 1];
    //let number = countdown[5];
    //println!("Number is {:?}", number); // this is beyond range array

    let number = countdown.get(5); // get can be used to return an Option enum
    let number = number.unwrap_or(&0) + 1;
    
    println!("Number is {:?}", number);

}

pub fn matching_option(){
    let countdown = [5, 4, 3, 2, 1];
    let number = countdown.get(5);
    let number = match number{
        Some(number) => number + 1,
        None => 0,
    };
    println!("Number is {:?}", number);
}

pub fn if_let(){
    let number = Some(13);
    if let Some(13) = number {
        println!("Thirteen")
    }
}

pub fn challenge(){
    enum Location{
        Unknown,
        Anonymous,
        Known(f64, f64),
    }

    impl Location{
        fn display(&self){
            match *self{
                Location::Unknown => println!("Unknown Location"),
                Location::Anonymous => println!("Anonymous Location"),
                Location::Known(lat, long) => println!("Location is at latitude {} and longitude {}", lat, long),
            }
        }

    }

    let address = Location::Unknown;
    address.display();

    let address = Location::Anonymous;
    address.display();

    let address = Location::Known(28.608295, -80.604177);
    address.display();
}