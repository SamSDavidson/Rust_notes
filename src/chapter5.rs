fn variable_scope() {
    let planet = "Earth"; // this variable is vaid in all of the variable_scop function
    let third_planet;
    println!("Planet: {}", planet);

    if true {
        let other_planet = "Jupiter"; // only valid in this block
        println!("Other Planet: {}", other_planet)
    }

    if true{
        third_planet = "Pluto";
    } else {
        third_planet = "";
    }
    //println!("Other Planet: {}"other_planet); // Will throw an error

    // The below will work as third planet is init at the top of the function
    println!("Third Planet: {}", third_planet);

}


fn var_mutability(){
    let planet = "Earth";
    println!("The Planet: {}", planet); // prints earth
    
    let planet = "Mars";
    println!("The Planet: {}", planet); // prints mars

    let planet = 4;
    println!("Ze Planet: {}", planet); //prints 4

    if true{
        let planet = "Saturn";
        println!("Planet in scope: {}", planet); // Saturn
    }
    println!("Planet out of scope: {}", planet); //4
}


fn string_data(){
    let mut message = String::from("Earth");
    println!("{}", message);
    message.push_str(" is home.");
    println!("{}", message);

}
fn move_cpy_clone(){
    let mut outer_planet: String;
    let powder_planet: String;
    {
        let mut inner_planet = String::from("Mercury"); // inner planet takes ownership of string
        println!("{}", inner_planet);
        outer_planet = inner_planet; // give ownership to outer_planet
        //println!("{}", inner_planet); --- Errors due to ownership change
        
        // we can allow the data to be cloned instead so they each have their own stored memory on the heap
        powder_planet = outer_planet.clone();
        println!("Owder!: {}\nP-Owder!: {}", outer_planet, powder_planet);
        
        // We can then impact the initial value or copy without impact the other
        outer_planet.clear();
        println!("Clear the heap!{}\nUse the Powder: {}",outer_planet, powder_planet);

    
    } // ownership dropped, memory freed
    // println!("{}", inner_planet); ---- No ownership / variable memory has been cleaned
    println!("{}", outer_planet);  //works
}

fn move_cpy_clone_num(){
    let outer_planet: i32;
    {
        let mut inner_planet = 1; 
        outer_planet = inner_planet; // rust implicitly copies the value, it does not clear the memmory for inner_planet
        inner_planet += 1; 
        println!("Inner Planet is {}", inner_planet);     
    } 
    println!("Outer planet: {}", outer_planet);
}

// interactions in a function - INT
fn transfer_of_ownership(){
    let rocket_fuel = 1;
    process_fuel(rocket_fuel);
    println!("rocket fuel is: {}", rocket_fuel);
}

fn process_fuel(mut propellant: i32) {
    propellant += 10;
    println!("Proppellant: {}", propellant);
}

//interactions in a func - STRING
fn transfer_of_ownership_str(){
    let rocket_fuel = String::from("RP-1");
    //ways to keep ownership
    process_fuel_str(rocket_fuel);
    //println!("rocket fuel is: {}", rocket_fuel); --> Error due to transfer of ownership
}

fn hold_str_ownership(){
    let rocket_fuel = String::from("RP-1");
    //ways to keep ownership
    process_fuel_str(rocket_fuel.clone());

    // have the function return a string
    let rocket_fuel = process_fuel_str_hold(rocket_fuel);
    process_fuel_str(rocket_fuel);
}

fn process_fuel_str(propellant: String) {
    println!("Proppellant: {}", propellant);
}
fn process_fuel_str_hold(propellant: String) -> String {
    println!("Propellant_Holder: {}", propellant);
    let new_fuel = String::from("LNG");
    new_fuel
}