// Traits
use std::any;
use std::fmt;

#[derive(PartialEq, PartialOrd)]
struct Satellite {
    name: String,
    velocity: f64, //miles per second
}

struct SpaceStation {
    name: String,
    crew_size: u8,
    altitude: u32,
}
trait Description {
    //method signatures that a type will need to have to implement
    fn describe(&self) -> String {
        String::from("An object flying through space!")
    }
}

impl Description for Satellite {
    // Implement default trait return
}

impl Description for SpaceStation {
    fn describe(&self) -> String {
        format!(
            "The {} flying at {} miles high with {} crep members on board!",
            self.name, self.altitude, self.crew_size
        )
    }
}
impl fmt::Display for Satellite{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
       write!(f,"{} flying at {}", self.name, self.velocity)
    }
}

pub fn implement_traits() {
    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72,
    };

    let iss = SpaceStation {
        name: String::from("ISS"),
        crew_size: 6,
        altitude: 254,
    };

    println!("Hubble: {}", hubble.describe());
    println!("ISS: {}", iss.describe());
}

pub fn derived_traits() {
    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72,
    };
    let gps = Satellite {
        name: String::from("GPS"),
        velocity: 2.42,
    };

    println!("huble > gps is {}", hubble > gps);
}

pub fn trait_bounds() {
    fn print_type<T: fmt::Debug>(item: T) {
        println!("{:#?} is {:#?}", item, any::type_name::<T>());
    }

    print_type(13);
    print_type(13.0);
    print_type("thirteen");
    print_type([13]);
}

pub fn multiple_trait_bounds() {
    //fn compare_and_print<T: fmt::Display + PartialEq + From<U>, U: fmt::Display + PartialEq + Copy>(a: T, b: U)
    fn compare_and_print<T, U>(a: T, b: U) 
        where T: fmt::Display + PartialEq + From<U>, 
              U: fmt::Display + PartialEq + Copy
     {
        if a == T::from(b) {
            println!("{} is equal to {}", a, b)
        } else {
            println!("{} is not equal to {}", a, b)
        }
    }

    compare_and_print(1.0, 1);
    compare_and_print(1.1, 1)
}

pub fn return_traits(){
    fn get_displayable(choice: bool) -> impl fmt::Display{
        if choice{
            13
        } else{
           42
        }
    }

    println!("{}", get_displayable(1 == 1))
}

pub fn challenge(){
    let hubble = Satellite{
        name: String::from("Hubble Telescope"),
        velocity: 4.72
    };

    println!("Name: {}", hubble);
}