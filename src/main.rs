use std::io;
use std::char;
use rand::distributions::{Distribution, Uniform};

fn get_guess() -> char {
    loop {
        println!("enter a single character: ") ;
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("could not read from stdin");
        match guess.to_ascii_uppercase().trim().parse::<char>(){ //remember to trim input to avoid enter spaces
            Ok(v) => return v,
            Err(e) => println!("could not understand input {}",e)
        }
    }
}

fn handle_guess(guess:char,correct:char)-> bool {
    if guess < correct {
        println!("go up");
        false

    } else if guess> correct {
        println!("go down");
        false
    } else {
        println!("you go it ..");
        true
    }
}


fn main() {
    println!("Welcome to guess game!");

    let step:Uniform<u8> = Uniform::new(65, 65+26);
    let mut rng = rand::thread_rng();
    let correct = step.sample(&mut rng) as char;

    loop {
        let guess = get_guess();
        if handle_guess(guess,correct){
            break;
        }
    }

}


