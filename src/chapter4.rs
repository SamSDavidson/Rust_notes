fn if_execution() {
    let x = 3;

    if x < 10 {
        println!("X is: {}", x);
    }

    // essentially
    if true {
        println!("True");
    }

    //expressions and arithmatic
    if x + 1 != 3 {
        println!("X + 1 is NOT 3");
    }
}

fn multiple_conditions() {
    let x = 3;
    let y = 5;

    if x > y {
        println!("x > y");
    } else if x == y {
        println!("x == y ");
    } else {
        println!("y > x");
    }

    // or nested
    if x > y {
        println!("Yes")
    } else {
        if x == y {
            println!("Eh");
        } else {
            println!("Yuck!");
        }
    }
}

fn conditional_assignment() {
    let make_x_odd = true;
    let x;

    if make_x_odd {
        x = 1;
    } else {
        x = 2;
    }
    println!("x is: {}", x);

    //Shorthand
    let y = if make_x_odd {1} else {2};
    println!("y: {}",y);
}

fn loop_conditions() {
    let mut count = 0;
    let result = loop {
        if count == 10 {
            break count * 10;
        }
        println!("Count is: {}", count);
        count += 1;
    };
    // The loop can update count as seen here
    println!("Loop ended");
    println!("The result is: {}", result);
}

fn while_loop() {
    let mut count = 0;
    while count < 10 {
        println!("Count {}", count);
        count +=1;
    }
    println!("Final Count is: {}", count);

    //iterate through array
    let ltr = ['a', 'b', 'c'];
    let mut ct = 0;
    while ct < ltr.len() {
        println!("Letter: {}", ltr[ct]);
        ct += 1;
    }
}

fn for_loop() {
    let message = ['h', 'e', 'l', 'l', 'o'];

    for item in message {
        println!("{}", item);
    }

    //Use iter and return index with enumerate
    for item in message.iter().enumerate() {
        println!("Index: {}\nChar: {}", item.0, item.1);
    }

    //or assign the iter key/index to values
    for(index, &item) in message.iter().enumerate() {
        println!("Index: {} - Item: {}", index, item);
        if item == 'e'{
            break;
        }
    }

    for number in 0..50 {
        println!("Number: {}", number);
    }
}

fn nesting_loops(){
    // commonly used for nesting over multi dimensional arrays
    let matrix = [[1,2,3],[4,5,6],[7,8,9]];

    //or for a nice printout
    for row in matrix {
        for num in row{
            print!("{}\t", num); // the \t adds a tab spacing
        }
        println!();
    }

    // update items in a loop
    for row in matrix {
        for mut num in row {
           num += 10;
            println!("Num is: {}", num);
        }
    }
}

fn challenge_code(){
    let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    let mut max: i32;
    let mut min: i32;
    let mut mean: f64;
    let length = numbers.len() as f64;
    let mut sum = 0;

    max = numbers[0];
    min = numbers[0];
    for number in numbers{
        sum += number;
        if number > max{
            max = number;
        } else if number < min {
            min = number;
        } else {
            continue
        }
        
    }
    println!("{}", sum);
    mean = sum as f64 / length;

    assert_eq!(max, 56);
    println!("Max Passed");
    assert_eq!(min, -18);
    println!("Min Passed.");
    assert_eq!(mean, 12.5);
    println!("All Tests Passed");

}
