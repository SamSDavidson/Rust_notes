// Error Handling
use std::fs;
use std::io;

use rand::thread_rng;
use rand::Rng;

pub fn unrecoverable_errors() {
    panic!("Houston, we've had a problem");
}

pub fn recoverable_errors() {
    let result = fs::read_to_string("the_ultimate_question.txt");

    let contents = match result {
        Ok(message) => message,
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => String::from("File not found"),
            io::ErrorKind::PermissionDenied => String::from("Permission denied"),
            _ => panic!("Houston, we have a problem {:?}", error),
        },
    };

    println!("The ultimate question is: {}", contents);
}

pub fn propogate_error() {
    fn read_and_combine(f1: &str, f2: &str) -> Result<String, io::Error> {
        let mut s1 = match fs::read_to_string(f2) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };
        let s2 = match fs::read_to_string(f2) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };
        s1.push('\n');
        s1.push_str(&s2);
        Ok(s1)
    }

    let result = read_and_combine("the_ultimate_question.txt", "the_ultimate_answer.txt");
    match result {
        Ok(message) => println!("The ultimate question is: {}", message),
        Err(error) => panic!("Houston, we have a problem {:?}", error),
    }

    // The ? operator can be used to simplify the propogation of errors
}

pub fn prop_error_short() {
    fn read_and_combine(f1: &str, f2: &str) -> Result<String, io::Error> {
        let mut s1 = fs::read_to_string(f2)?;
        let s2 = match fs::read_to_string(f2) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };
        s1.push('\n');
        s1.push_str(&s2);
        Ok(s1)
    }
}
// CHALLENGE
pub fn challenge() {
    higher_or_lower();
}

pub fn higher_or_lower() {
    // Challenge: Gen random number and play high or low

    let mut win = false;
    let mut rand_num = thread_rng().gen_range(1..101);
    let mut guess_count = 0;

    while !win {
        if guess_count >= 5 {
            println!("Game loss after 5 guesses!\nTry again? (y/n)");
            let mut retry_opt = String::new();
            let _ = io::stdin().read_line(&mut retry_opt);
            if retry_opt.trim() == "y" || retry_opt.trim() == "Y" {
                guess_count = 0;
                rand_num = thread_rng().gen_range(1..101);
                continue;
            }
            win = true;
            break;
        }
        let mut guess = String::new();
        match io::stdin().read_line(&mut guess) {
            Ok(s) => s.to_string(),
            Err(_) => {
                println!("Enter a value");
                continue;
            }
        };
        let num_guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };

        if num_guess < rand_num {
            println!("Too low!");
            guess_count += 1;
        } else if num_guess > rand_num {
            println!("Too high!");
            guess_count += 1;
        } else {
            println!("You win!!!\nSolution found in {} guesses", guess_count);
            win = true;
        }
    }
}
