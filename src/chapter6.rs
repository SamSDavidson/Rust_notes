use std::io;
use rand::prelude::*;

pub fn standard_input(){
    //apart of std::io
    let mut buffer = String::new();
    println!("Enter a message:");

    // access standard input and read line
    let _ = io::stdin().read_line(&mut buffer); // when the line reads input, it updates buffer
    println!("buffer is {}", buffer);
}

pub fn parse_strings(){

    let mut value = String::new();
    let _ = io::stdin().read_line(&mut value); 
    let number = value.trim().parse::<i32>().unwrap(); 
    // standard in includes a new line that must be removed
    //turbofish operatior ::<datatype>()
    // explicitly naming the type will work as well/ number :i32 = value.trim().parse()

    // This will return a results
    println!("Number + 1 is {}", number + 1);

    //using the prelude option
    let thread_count = thread_rng().gen_range(1..11);
    println!("{}", thread_count)

}

pub fn random_gen(){
    let number = random::<f64>();
    println!("{}",number);
}

pub fn higher_or_ower(){
    // Challenge: Gen random number and play high or low
   
    let mut win = false;
    let mut rand_num = thread_rng().gen_range(1..101);
    let mut guess_count = 0;
    while !win{
        if guess_count >= 5{
            println!("Game loss after 5 guesses!\nTry again? (y/n)");
            let mut retry_opt = String::new();
            let _ = io::stdin().read_line(&mut retry_opt);
            if retry_opt.trim() == "y" || retry_opt.trim() == "Y"{
                guess_count = 0;
                rand_num = thread_rng().gen_range(1..101);
                continue;
            } else {
                win = true;
            }
        }
        let mut guess = String::new();
        let _ = io::stdin().read_line(&mut guess);
        let num_guess: i32 = guess.trim().parse()
            .ok()
            .expect("Expected a number");
        if num_guess < rand_num{
            println!("Too low!");
            guess_count += 1;
        }else if num_guess > rand_num{
            println!("Too high!");
            guess_count += 1;
        } else {
            println!("You win!!!\nSolution found in {} guesses", guess_count);
            win = true;
        }
    }
}