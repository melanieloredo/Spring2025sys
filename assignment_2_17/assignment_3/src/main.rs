use std::io;

fn main() {

    let mut secret = 75;
    let mut num_guesses = 0;
    //loop for guess
    loop {
        //user's guess
    let mut guess = String::new();
        //prompt user
    println!("Enter your guess: ");
        //user input
    io::stdin().read_line(&mut guess).expect("Failed");

    //convert input to integer
    let guess: i32 = match guess.trim().parse() {
        //handle the error
        Ok(num) => num,
        Err(_) => {
            println!("not a valid");
            return; // Exit if conversion fails
        }};
    
    //update number guesses
    num_guesses += 1;

    //check guess
    let check = check_guess(guess, secret);
    
    //print hints
    if check == 1{
        println!("Your guess is too high!");
    }else if check == -1{
        println!("Your guess is too low!");
    }else{
        println!("You're correct!");
        break;
    }
    };

    //print number of guesses
    println!("Number of guesses: {}", num_guesses);
}

//function to check guesses
fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret{
        0
    }else if guess > secret{
        1
    }else{
        -1
    }
}