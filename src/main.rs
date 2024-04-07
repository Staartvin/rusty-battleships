use std::io;

use rand::Rng;
use strum::{IntoEnumIterator, VariantArray};

use crate::battleship::{Battleship, BattleshipName};
use crate::board::{Board, BoardPosition};

mod battleship;
mod board;

fn main() {
    println!("Welcome to Battleships!");
    println!("The goal in this game is to find the enemy's battleships and destroy them.");

    println!("Let's first place your own battleships...");

    let mut player_ships: Vec<Battleship> = Vec::with_capacity(BattleshipName::VARIANTS.len());
    let board = Board {
        x_size: 5,
        y_size: 5,
    };

    for ship_name in BattleshipName::iter() {
        loop {
            let mut ship_size_input = String::new();
            println!("Provide size for ship '{:#?}'", ship_name);

            if let Err(_) = io::stdin().read_line(&mut ship_size_input) {
                println!("Failed to read size, please try again");
                continue;
            }

            ship_size_input = ship_size_input.trim_end().to_string();

            let Ok(ship_size) = ship_size_input.parse::<u16>() else {
                println!("Cannot parse {ship_size_input} to integer!");
                continue;
            };

            loop {
                let mut start_position_input = String::new();
                println!("Provide start position for ship '{:#?}'", ship_name);

                if let Err(_) = io::stdin().read_line(&mut start_position_input) {
                    println!("Failed to read position, please try again");
                    continue;
                }

                start_position_input = start_position_input.trim_end().to_string();

                let Some(ship_position) = BoardPosition::from_string(&start_position_input) else {
                    println!("Cannot parse '{start_position_input}' to position!");
                    continue;
                };

                if let Err(error_message) = board.is_position_on_board(&ship_position) {
                    println!(
                        "Position '{start_position_input:?}' does not fit on the board: {error_message}"
                    );
                    continue;
                }
            }
        }
    }

    println!("All your ships have been placed. Let's play the game!");
}

fn generate_empty_ship_set() -> Vec<Battleship> {
    let mut ships: Vec<Battleship> = Vec::new();
    let mut rng = rand::thread_rng();

    for ship_name in BattleshipName::iter() {
        let ship_size = rng.gen_range(1..6);
        println!(
            "Generating random size for {:#?}: {:#?}",
            ship_name, ship_size
        );
        ships.push(Battleship {
            name: ship_name,
            size: ship_size,
        })
    }

    ships
}
