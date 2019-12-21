extern crate rand;
extern crate text_io;
extern crate clap;

use rand::Rng;
use std::cmp::Ordering;
use std::io;
use clap::{Arg, App};


fn game_loop(tries: &mut i32, rng: i32){
    loop {

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => {
                *tries = *tries+1;
                num
            },
            Err(_) => {
                println!("Guess Again");
                continue;
            },
        };


        match guess.cmp(&rng) {
            Ordering::Less    => println!("Too LOW"),
            Ordering::Greater => println!("Too HIGH"),
            Ordering::Equal   => {
                println!("Congratulations, you guess Correctly!");
                break;
            }
        }
    }
}

fn get_arg_limit(s: &str, limit : &mut i32){
    match s.parse::<i32>(){
        Ok(n) => *limit = n+1,
        Err(_) => (),
    }
}

fn main() {

    let matches = App::new("Guess my Number").arg(Arg::with_name("num")
                                                  .short("n")
                                                  .long("number")
                                                  .takes_value(true)
    ).get_matches();
    
    let num_str = matches.value_of("num");
    let mut limit : i32 = 11;
    match num_str{
        None => (),
        Some(s) => {
            get_arg_limit(s, &mut limit)
        }
    }
    let number = rand::thread_rng().gen_range(1,limit);
    println!("Guess my Number!");
    let mut tries : i32 =0;
    game_loop(&mut tries, number);
    println!("It took you {} tries", tries);
    return;
    

    

   

}
