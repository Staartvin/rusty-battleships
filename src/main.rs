use std::io;
use std::str::FromStr;

use rand::Rng;
use strum::{IntoEnumIterator, VariantArray};

use crate::battleship::{Battleship, BattleshipName};
use crate::board::{Board, BoardPosition, PlacementDirection};

mod battleship;
mod board;

fn main() {
    println!("Welcome to Battleships!");
    println!("The goal in this game is to find the enemy's battleships and destroy them.");

    println!("Let's first place your own battleships...");

    let allowed_chars = BoardPosition::get_allowed_characters();
    let mut board = Board {
        x_size: allowed_chars.iter().count() as u16,
        y_size: allowed_chars.iter().count() as u16,
        player_ships: Vec::with_capacity(BattleshipName::VARIANTS.len()),
    };

    'ship_loop: for ship_name in BattleshipName::iter() {
        loop {
            println!("-----");
            println!("Current board:");
            board.print_board();
            println!("-----");

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
                println!("Provide start position for ship '{ship_name:#?}'");

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

                if board.has_ship_at_cell(&ship_position) {
                    println!("There is already a ship on position '{start_position_input:#?}'");
                    continue;
                }

                loop {
                    let mut ship_direction_input = String::new();
                    println!("In which cardinal direction is this ship placed (North, East, South, West)? ");

                    if let Err(_) = io::stdin().read_line(&mut ship_direction_input) {
                        eprintln!("Failed to read direction, please try again");
                        continue;
                    }

                    ship_direction_input = ship_direction_input.trim_end().to_string();

                    let Ok(ship_direction) =
                        PlacementDirection::from_str(&ship_direction_input.to_uppercase())
                    else {
                        eprintln!("Cannot parse '{ship_direction_input}' to direction!");
                        continue;
                    };

                    match board.compute_ship_coordinates(&ship_position, ship_size, &ship_direction)
                    {
                        Ok(ship_coordinates) => {
                            board.player_ships.push(Battleship {
                                name: ship_name,
                                size: ship_size as u32,
                                coordinates: ship_coordinates,
                            });

                            eprintln!("Successfully placed {ship_name:?}!");

                            continue 'ship_loop;
                        }
                        Err(message) => {
                            eprintln!("Ship could not be placed in that direction: {message}");
                            continue;
                        }
                    }
                }
            }
        }
    }
    println!("All your ships have been placed. Let's play the game!");
}
