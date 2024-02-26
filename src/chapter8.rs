#[derive(Debug)]
#[derive(Clone)]
pub struct Shuttle{
    name: String,
    crew_size: u8,
    propellant: f64,
}
// implementation block
impl Shuttle{
    fn get_name(&self) -> &str{
        return &self.name
    }
    fn add_fuel(&mut self, gallons: f64){
        self.propellant += gallons;
    }

    // associated function
    fn new(name: &str) -> Shuttle{
        Shuttle { name: String::from(name), crew_size: 7, propellant: 0.0 }
    }
}

pub fn define_structs(){
    let mut vehicle = Shuttle{
        name: String::from("Endeavour"),
        crew_size: 7,
        propellant: 835958.0,
    };
    println!("Name: {}\nCrew: {}\nPropellant: {}", vehicle.name, vehicle.crew_size, vehicle.propellant);
    vehicle.name = String::from("Atlantis");
    println!("{:?}", vehicle)
}

pub fn struct_update(){
    let vehicle = Shuttle{
        name: String::from("Endeavour"),
        crew_size: 7,
        propellant: 835958.0,
    };
    let vehicle2 = Shuttle {
        name: String::from("Discovery"),
        ..vehicle
    };
    let vehicle3 = Shuttle{
        ..vehicle.clone()
    };
    println!("Vehicle: {:?}", vehicle);
    println!("Vehicle 2: {:?}", vehicle2);
    println!("Vehicle 3: {:?}", vehicle3);
    
}

pub fn struct_methods(){
    let mut vehicle = Shuttle{
        name: String::from("Endeavour"),
        crew_size: 7,
        propellant: 835958.0,
    };
    let vehicle_name = vehicle.get_name();
    println!("{}", vehicle_name);
    println!("Starting Propellant: {}", vehicle.propellant);
    // add fuel
    vehicle.add_fuel(164042.0);
    println!("Updated Propellant: {}", vehicle.propellant);
}

pub fn associated_function(){
 let mut vehicle = Shuttle::new("Enterprise");
 let vehicle_name = vehicle.get_name();
 println!("{}", vehicle_name);
 vehicle.add_fuel(164042.0);
}
// Tuple Struct
struct Color(u8, u8, u8); // Color RGB tuple struct
struct Point(u8,u8,u8); // XYZ coordinates

fn get_y(p: Point) -> u8{
    p.1
}
pub fn tuple_structs(){
    let red = Color(255, 0, 0);
    let a_point = Point(4,5,6);
    // access value
    println!("Element 1: {}\nElement 2: {}\nElement 3: {}",red.0,red.1, red.2);
    let y = get_y(a_point);
    println!("Y is: {}", y);

}
struct Rectangle{
    width: f64,
    height: f64,
}
impl Rectangle{
    fn get_area(&self) -> f64{
        self.width * self.height
    }
    fn scale(&mut self, scaler: f64){
        self.width *= scaler;
        self.height *= scaler;
    }

    fn new(width: f64, height: f64) -> Rectangle{
        Rectangle{
            width, 
            height,
        }
    }
}
pub fn struct_challenge(){

    let mut rect = Rectangle::new(1.2, 3.4);
    assert_eq!(rect.get_area(), 4.08);
    rect.scale(0.5);
    assert_eq!(rect.get_area(), 1.02);
    println!("Tests passed!");

}