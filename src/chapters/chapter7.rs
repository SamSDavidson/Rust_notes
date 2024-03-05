use std::env;
use std::fs;
use std::io::prelude::*;

pub fn cli_args() {
    if env::args().len() < 2 {
        println!("Not enough arguments");
    } else {
        for (index, args) in env::args().enumerate() {
            println!("Arugment{} is {}", index, args)
        }

        // specific argument
        let arg2 = env::args().nth(2).unwrap();
        println!("Argument 2: {}", arg2);
    }
}

pub fn reading_files() {
    //declare variable to hold file contents
    let contents = fs::read_to_string("planet.txt").unwrap();
    // will return a result enum type
    println!("{}", contents);

    for line in contents.lines() {
        println!("Line is {}", line);
    }

    // read with non text data using bytes
    let contents = fs::read("planet.txt").unwrap();
    println!("{:?}", contents)
}

pub fn writing_files() {
    let mut speech = String::new();
    speech.push_str("We choose to go to the Moon in this decade\n");
    speech.push_str("and do other things,\n");
    speech.push_str("not because they are easy,\n");
    speech.push_str("but because they are hard.");

    // takes path and contents
    fs::write("speech.txt", speech).expect("Invalid path");

    // Append
    let mut file = fs::OpenOptions::new()
        .append(true)
        .open("planet.txt")
        .unwrap();
    // creates a blank set of options to configure how a file is opened
    //  set file path -> use unwrap due to expect

    //add content to append
    //advise write trait to handle data as bytes using 'b'
    file.write(b"\nPluto").expect("File does not exist");
}

pub fn challenge_io() {
    if env::args().len() < 2 || env::args().nth(1) == None || env::args().nth(2) == None {
        println!("Must include two arguments <filepath> <search name>");
        std::process::exit(1);
    } else {
        let file_path = env::args().nth(1).unwrap();
        let name_data = env::args().nth(2).unwrap();

        let contents = fs::read_to_string(file_path).unwrap();
        for line in contents.lines() {
            if name_data == line {
                println!("Name {} is found!", name_data);
                return;
            }
        }
        println!("Name: {} was not found", name_data);
    }
}
