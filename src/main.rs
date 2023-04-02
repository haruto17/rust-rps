use proconio::input;
use rand::Rng;

fn _player_input() -> String {
    input! {
        _p: String,
    }

    _p
}

fn main() {
    println!("{}", "R -> Rock 🪨");
    println!("{}", "P -> Paper 📄");
    println!("{}", "S -> Scissors ✂️");
    println!();

    println!("{}", "Which one would you choose?");

    let mut _player;

    loop {
        _player = _player_input();
        match &_player[..] {
            "R" => {
                break;
            }
            "P" => {
                break;
            }
            "S" => {
                break;
            }
            _ => {
                println!("{}", "Cant'use");
                continue;
            }
        }
    }

    println!();

    let mut _rng = rand::thread_rng();
    let mut _cp = _rng.gen_range(1..4);

    println!("You => {}",_player);

    // 1 == R , 2 == P , 3 == S
    if _cp == 1 {
        println!("CP => {}","Rock 🪨");
        println!();
        if _player == "R" {
            println!("{}", "Draw 😗");
        } else if _player == "P" {
            println!("{}", "You win 😎");
        } else if _player == "S" {
            println!("{}", "You lose 😥");
        }
    } else if _cp == 2 {
        println!("CP => {}","Paper 📄");
        println!();
        if _player == "R" {
            println!("{}", "You lose 😥");
        } else if _player == "P" {
            println!("{}", "Draw 😗");
        } else if _player == "S" {
            println!("{}", "You win 😎");
        }
    } else if _cp == 3 {
        println!("CP => {}","Scissors ✂️");
        println!();
        if _player == "R" {
            println!("{}", "You win 😎");
        } else if _player == "P" {
            println!("{}", "You lose 😥");
        } else if _player == "S" {
            println!("{}", "Draw 😗");
        }
    }
}
