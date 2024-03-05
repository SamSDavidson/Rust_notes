fn array_types() {
    let letters = ['a', 'b', 'c'];
    println!("Print first letter: {}", letters[0]);

    // a mutable array
    let mut ltrs = ['b', 'c', 'd'];
    // change letter
    ltrs[1] = 'x';

    //If we specify the data type and length we can create an empty array
    let numbers: [i32; 5];

    // repeat expression - create five copies of zero in the array
    numbers = [0; 5];

    println!("last number is {}", numbers[4]);

    let index = numbers.len();
    println!(
        "Trying to run the fifth variable (last is 4) {}",
        numbers[index]
    );
    // The above provides an out of bounds exception

    
}

fn multidimensional_arrays() {
    let parking_lot = [[1, 2, 3], [4, 5, 6]];
    println!("Row 1, spot 2: {}", parking_lot[0][1]);

    let parking_garage = [[[1, 2, 3], [4, 5, 6]]];
    let garage: [[[i32; 100]; 20]; 5];
}
fn tuple_types(){
    //initialize tuples with ()
    let stuff = (10, 3.14, 'x');

    //access items
    let first_item = stuff.0;
    let second_item = stuff.1; //etc...
    println!("{} {}", first_item, second_item);
    // type declarations
    let mut other_stuff: (u8, f32, char) = (9, 33.4, 'b');

    //changing items
    other_stuff.0 += 3; //adds
    other_stuff.2 = 'r'; //changes

    // sets the values of a, b, and c to the values stored within stuff
    //Known as deconstructing
    let (a, b, c) = stuff;
    println!("Tuple it up {}, {}, {}",a, b, c );

}