extern crate rand;
extern crate text_io;

use rand::Rng;
use std::collections::HashMap;
use std::io;

fn initialise_alphabet() -> HashMap<char, bool> {
    let mut alphabet = HashMap::new();
    let letters = (b'A'..=b'Z') // Start as u8
        .filter_map(|c| {
            let c = c as char; // Convert to char
            if c.is_alphabetic() {
                Some(c)
            } else {
                None
            } // Filter only alphabetic chars
        })
        .collect::<Vec<_>>();
    for l in letters {
        alphabet.insert(l, false);
    }
    return alphabet;
}

fn update_alphabet(alphabet: &mut HashMap<char, bool>, letter: char, word : &str,tries: &mut i32) {
    let cond = alphabet.get(&letter);
    let mut correct : bool = true;
    match cond {
        Some(s) =>{
            if !s {
                for l in word.chars(){
                    if l != letter{
                        correct = false;
                    }
                    else {
                        correct = true;
                        break;
                    }
                }
                *alphabet.get_mut(&letter).unwrap() = true;
            }
        },
        None => ()
    }
    if !correct{
        *tries = *tries -1;
    }
}

fn print_word(word: &str, alphabet: &HashMap<char, bool>) {
    for l in word.chars() {
        match alphabet.get(&l) {
            Some(a) => {
                if *a {
                    print!("{}", l);
                } else {
                    print!("_");
                }
            }
            None => (),
        }
    }
    println!();
}

fn check_letters(word: &str, alphabet: &HashMap<char, bool>) -> bool {
    for l in word.chars() {
        match alphabet.get(&l) {
            Some(a) => {
                if !*a {
                    return false;
                }
            }
            None => (),
        }
    }
    return true;
}

fn get_guess() -> char {
    let mut guess :String; 
    loop {
        guess = String::new();
        println!("Guess a letter!");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess = guess.trim();
        if guess.len() != 1 {
            println!("Error, enter a single character");
            continue;
        }
        let guess: char = guess.chars().next().unwrap();
        if guess.is_numeric() {
            println!("Error, enter a character");
            continue;
        }
        break;
    }
    return guess.to_uppercase().to_string().chars().next().unwrap();
}

fn print_lives(tries: &mut i32){
    match tries{
            6 => {
                println!(" +---+");
                println!(" |   |");
                println!("     |");
                println!("     |");
                println!("     |");
                println!("=========");
            }
            5 => {
                println!(" +---+");
                println!(" |   |");
                println!(" O   |");
                println!("     |");
                println!("     |");
                println!("=========");
            }
            4 => {
                println!(" +---+");
                println!(" |   |");
                println!(" O   |");
                println!(" |   |");
                println!("     |");
                println!("=========");
            }
            3 => {
                println!(" +---+");
                println!(" |   |");
                println!(" O   |");
                println!("/|   |");
                println!("     |");
                println!("=========");
            }
            2 => {
                println!(" +---+");
                println!(" |   |");
                println!(" O   |");
                println!("/|\\  |");
                println!("     |");
                println!("=========");
            }
            1 => {
                println!(" +---+");
                println!(" |   |");
                println!(" O   |");
                println!("/|\\  |");
                println!("/    |");
                println!("=========");
            }
            _ => ()
        }
}

fn game_loop(mut alphabet: HashMap<char, bool>, word: &str, tries: &mut i32) -> bool{
    loop {
        
        print_lives(tries);
        print_word(word, &alphabet);
        let guess = get_guess();
        update_alphabet(&mut alphabet, guess, word, tries);

        let finished = check_letters(word, &alphabet);
        if finished {
            return true;
        }
        if *tries == 0 {
            return false;
        }
    }
}

fn main() {
    let dict = ["ALPHABET", "TEST", "CAKE", "EMACS"];
    let rng = rand::thread_rng().gen_range(0, dict.len());
    let word = dict[rng];
    let mut tries: i32 = 6;

    let alphabet = initialise_alphabet();
    let victory = game_loop(alphabet, word, &mut tries);

    println!("{}", word);
    if victory {
        println!("You had {} attempts left", tries);
    }
    else{
        println!(" +---+");
        println!(" |   |");
        println!(" O   |");
        println!("/|\\  |");
        println!("/ \\  |");
        println!("=========");
        println!("Game Over, you ran out of lives");
    }
}
