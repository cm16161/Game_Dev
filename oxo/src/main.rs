extern crate text_io;

use std::io;

#[derive(Clone)]
struct Coordinate {
    x: i32,
    y: i32,
}

#[derive(Clone)]
enum Player {
    X,
    O,
}

impl Copy for Coordinate {}
impl Copy for Player {}

fn get_coord() -> i32 {
    let mut input: String;
    loop {
        input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        match input.trim().parse() {
            Ok(num) => {
                if num >= 0 && num < 3 {
                    return num;
                } else {
                    println!("Try Again");
                    continue;
                }
            }
            Err(_) => {
                println!("Try Again");
                continue;
            }
        };
    }
}

fn get_row() -> i32 {
    println!("Enter a row");
    return get_coord();
}

fn get_col() -> i32 {
    println!("Enter a column");
    return get_coord();
}

fn validate_coord(coord: Coordinate, world: [[&str; 3]; 3]) -> bool {
    if world[coord.x as usize][coord.y as usize] == " " {
        return true;
    } else {
        return false;
    }
}

fn place_token(turn: Player, coord: Coordinate, world: &mut [[&str; 3]; 3]) {
    match turn {
        Player::X => world[coord.x as usize][coord.y as usize] = "X",
        Player::O => world[coord.x as usize][coord.y as usize] = "O",
    }
}

fn swap_player(player: &mut Player) {
    match player {
        Player::X => {
            *player = Player::O;
        }
        Player::O => {
            *player = Player::X;
        }
    }
}

fn print_board(world: [[&str;3];3]){
    println!("-------------");
    for i in 0..3 {
        for j in 0..3 {
            print!("| {} ", world[i][j]);
        }
        println!("|");
        println!("-------------");
    }
}

fn check_victory(world: [[&str;3];3]) -> bool {
    for i in 0..3{
        if (world[i][0] == world[i][1]) && (world[i][0] == world[i][2]) && world[i][0] != " "{
            return true;
        }
        if(world[0][i] == world[1][i]) && (world[0][i] == world[2][i]) && world[0][i] != " "{
            return true;
        }
    }
    if(world[0][0] == world[1][1]) && (world[0][0] == world[2][2]) && (world[0][0] != " "){
        return true;
    }
    if(world[2][0] == world[1][1]) && (world[2][0] == world[0][2]) && (world[2][0] != " "){
        return true;
    }
    return false;
    
}

fn get_coordinates(mut coordinate : Coordinate, world : [[&str;3];3]) -> Coordinate{
    loop {
        coordinate.x = get_row();
        coordinate.y = get_col();
        let valid = validate_coord(coordinate, world);
        if valid {
            return coordinate;
        }
        else {
            println!("Try Again");
        }
    }
}

fn game_loop(mut world: [[&str;3];3],mut player: Player){
    print_board(world);
    loop {
        let mut coordinate = Coordinate { x: -1, y: -1 };
        coordinate = get_coordinates(coordinate, world);
        
        
        place_token(player, coordinate, &mut world);
        print_board(world);
        if check_victory(world){
            break;
        }
        else{
            swap_player(&mut player);
            continue;
        }
    }
}

fn main() {
    let world = [[" "; 3]; 3];
    let player: Player = Player::O;
    
    game_loop(world,  player);

    println!("Game Over");
    match player{
        Player::X => println!("X's Wins!"),
        Player::O => println!("O's Wins!"),
    }
}
