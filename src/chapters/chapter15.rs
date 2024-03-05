//Collections
use std::collections::HashMap;

pub fn vector_notes(){
    let mut astronauts: Vec<String> = Vec::new();
    astronauts.push(String::from("Shepard"));
    astronauts.push(String::from("Grissom"));
    astronauts.push(String::from("Glenn"));

    println!("Astronauts: {:?}", astronauts);

    // let third = &astronauts[2];
    // println!("Third astronaut: {}", third);

    let last = astronauts.pop();
    println!("Last astronaut: {:?}", last);

    let third = astronauts.get(2);
    print!("Third astronaut: {:?}", third);
}

pub fn hash_map(){
    let mut missions_flown = HashMap::new();
    missions_flown.insert("Hadfield", 3);
    missions_flown.insert("Hurley", 3);
    missions_flown.insert("Barron", 0);

    //Overwrite  KV Pair
    missions_flown.insert("Barron", 1);

    //enter a new KV pair if value does not exist
    missions_flown.entry("Barron").or_insert(2);
    missions_flown.entry("Stone").or_insert(2);

    //Update a value based on the old value
    // or_insert returns a reference to the value that can be altered
    let kayla = missions_flown.entry("Barron").or_insert(0);
    *kayla += 1; //dereferences and increments value

    let barron_missions = missions_flown.get("Barron");
    println!("Barron: {:?}", barron_missions);
    println!("Missions Flown: {:?}", missions_flown);
}

