// Lifetimes

pub fn lifetime_annotate() {
    fn best_fuel<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let result;
    let propellant1 = String::from("RP-1");
    let propellant2 = String::from("LNG");
    result = best_fuel(&propellant1, &propellant2);
    println!("Result: {}", result);
}

pub fn multiple_annotations() {
    fn best_fuel<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            x
        }
    }

    let result;
    let propellant1 = String::from("RP-1");
    {
        let propellant2 = String::from("LNG");
        result = best_fuel(&propellant1, &propellant2);
    }
    println!("Result: {}", result);
}

pub fn struct_lifetime() {
    struct Shuttle<'a> {
        name: &'a str,
    }
    impl<'a, 'b> Shuttle<'a> {
        fn send_transmission(&'a self, msg: &'b str) -> &'b str {
            println!("Transmitting message: {}", msg);
            // self.name // returns name as string
            msg
        }
    }

    let vehicle = Shuttle { name: "Endeavor" };
    let sender = vehicle.send_transmission("Greetings from orbit"); //transmits greeting
    println!("Sender is {}", sender); // prints returned name
}
