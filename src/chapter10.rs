{ // Borrowing References
    fn borrowing_references() {
        let rocket_fuel = String::from("RP-1");
        let length = process_fuel(&rocket_fuel);

        println!("rocket_fuel is {}", rocket_fuel);
        println!("That word is {} characters", length);
    }

    fn process_fuel(propellant: &String) -> usize {
        // this is the less optimal way to manage
        println!("Processing Propellant {}", propellant);
        let length = propellant.len();
        length
    }
}
{ // Mutable References
    fn mutable_references() {
        let mut rocket_fuel = String::from("RP-1");
        let length = process_fuel(&mut rocket_fuel);

        println!("rocket_fuel is {}", rocket_fuel);
        println!("That word is {} characters", length);
    }

    fn process_fuel(propellant: &mut String) -> usize {
        // this is the less optimal way to manage
        println!("Processing Propellant {}", propellant);
        propellant.push_str(" is highly flammable");
        let length = propellant.len();
        length
    }
}
{ // Dangling References
    fn dangling_refs() {
        let rocket_fuel = produce_fuel();
        println!("The rocket fuel is {}", rocket_fuel);
    }
    // Remove refernces to pass ownership
    // A ref to new_fuel cannot exist outside the function scope
    fn produce_fuel() -> String {
        let new_fuel = String::from("RP-1");
        new_fuel
    }
}
{ //Reference Slices
    fn slice_ref() {
        let message = String::from("Greetings from earth!");
        println!("Message is: {}", message);

        // Creates a slice starting at index 15, to 15+5
        // This should be the length of the word 'earth'
        let last_word = &message[15..15 + 5];
        println!("Last word: {}", last_word);

        // Or all ending characters
        let last_section = &message[15..];
        println!("{}", last_section);

        // Slices in arrays
        let planets = [1, 2, 3, 4, 5, 6, 7, 8];
        let inner_planets: &[i32] = &planets[..4];
        print!("Inner planets are: {:?}", inner_planets);
    }
}
{ // Slice as input to funciton

    //First example is referencing an entire word and sendit back only a slice
    fn slice_as_param() {
        let message = String::from("Greetings from Earth!");
        let first_word = get_first_word(&message);

        println!("The first word is: {}", first_word);
    }

    fn get_first_word(s: &String) -> &str {
        let bytes = s.as_bytes(); //convert string into bytes
        for (index, &item) in bytes.iter().enumerate() {
            //iterate through bytes
            if item == b' ' {
                //if a space is found, return everything up to that space
                return &s[..index]; // found a space
            }
        }
        &s //returns entire word if no space is found
    }

    fn slice_as_param() {
        let message = String::from("Greetings from Earth!");

        let first_word = get_first_word(&message[10..]);
        //-- This will fail while the func expects &String, and is receiving an &str

        println!("The first word is: {}", first_word);
    }

    fn get_first_word(s: &str) -> &str {
        let bytes = s.as_bytes();
        for (index, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[..index];
            }
        }
        &s
    }
}
{
    fn challenge_issued() {
        let test1 = "We need more space.";
        assert_eq!(trim_spaces(test1), "We need more space.");

        let test2 = String::from("   There's space in front.");
        assert_eq!(trim_spaces(&test2), "There's space in front.");

        let test3 = String::from("There's space to the rear. ");
        assert_eq!(trim_spaces(&test3[..]), "There's space to the rear.");

        let test4 = "  We're surrounded by space!    ";
        assert_eq!(trim_spaces(test4), "We're surrounded by space!");

        let test5 = "     ";
        assert_eq!(trim_spaces(test5), "");

        let test6 = "";
        assert_eq!(trim_spaces(test6), "");

        let test7 = " 🚀 ";
        assert_eq!(trim_spaces(test7), "🚀");
        println!("Tests passed!");
    }

    fn trim_spaces(no_space: &str) -> &str {
        let bytes = no_space.as_bytes();
        let mut first_char: usize = 0;
        let mut last_char: usize = if no_space.len() > 0 {
            no_space.len() as usize - 1
        } else {
            0
        };

        for (index, &item) in bytes.iter().enumerate() {
            if item != b' ' {
                first_char = index;
                break;
            }
        }
        for (index, &item) in bytes.iter().enumerate().rev() {
            if item != b' ' {
                last_char = index + 1;
                break;
            } else {
                last_char = 0;
            }
        }
        return &no_space[first_char..last_char];
    }
    fn instructor_trim_spaces(s: &str)->{
        let mut start = 0;
        for (index, character) in s.chars().enumerate() {
            if character != ' '{
                start = index;
                break;
            }
        }

        // search reverse for last non-space
        let mut end = 0;
        for (index, character) in s.chars().rev().enumerate() {
            if character != ' '{
                end = s.len() - index;
                break;
            }
        }
        &s[start..end]
    }

}
