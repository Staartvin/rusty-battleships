use strum::EnumString;
use unicode_segmentation::UnicodeSegmentation;

use crate::battleship::Battleship;

pub struct Board {
    pub x_size: u16,
    pub y_size: u16,
    pub player_ships: Vec<Battleship>,
}

#[derive(Debug)]
pub struct BoardPosition {
    pub x_position: u16,
    pub y_position: u16,
}

#[derive(EnumString, PartialEq, Eq)]
pub enum PlacementDirection {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

impl BoardPosition {
    /// Generates a [BoardPosition] from a given string. The input is expected to be a character
    /// from the alphabet (e.g. 'C') concatenated with an integer (e.g. 10): C10.
    pub fn from_string(position_string: &str) -> Option<BoardPosition> {
        if position_string.graphemes(true).count() < 2 {
            return None;
        }

        let allowed_chars = Self::get_allowed_characters();

        let first_character = &position_string[..1];

        // Parse first character
        let x_pos: u16 = match allowed_chars.iter().position(|c| c == first_character) {
            None => {
                eprintln!("The character '{first_character} was not a valid character!'");
                return None;
            }
            Some(index) => index + 1,
        } as u16;

        // Parse second character
        let second_character = &position_string[1..];

        let Ok(y_pos) = second_character.parse::<u16>() else {
            eprintln!("Could not parse '{second_character} into an integer!'");
            return None;
        };

        return Some(BoardPosition {
            x_position: x_pos,
            y_position: y_pos,
        });
    }

    pub fn get_allowed_characters() -> Vec<String> {
        return vec![
            "A".to_string(),
            "B".to_string(),
            "C".to_string(),
            "D".to_string(),
            "E".to_string(),
            "F".to_string(),
            "G".to_string(),
        ];
    }
}

impl Board {
    pub fn is_position_on_board(&self, position: &BoardPosition) -> Result<(), String> {
        return if position.x_position > self.x_size || position.x_position <= 0 {
            Err("X position is out of bounds".to_string())
        } else if position.y_position > self.y_size || position.y_position <= 0 {
            Err(format!(
                "Y position is out of bounds: Y {}, max Y = {}",
                position.y_position, self.y_size
            ))
        } else {
            Ok(())
        };
    }

    pub fn compute_ship_coordinates(
        &self,
        start_position: &BoardPosition,
        ship_size: u16,
        placement_direction: &PlacementDirection,
    ) -> Result<Vec<BoardPosition>, String> {
        let mut positions: Vec<BoardPosition> = Vec::with_capacity(ship_size as usize);

        if placement_direction == &PlacementDirection::NORTH
            && start_position.y_position < ship_size
        {
            return Err("Out of bounds on north side of the board".to_string());
        } else if placement_direction == &PlacementDirection::WEST
            && start_position.x_position < ship_size
        {
            return Err("Out of bounds on west side of the board".to_string());
        }

        for index in 0..ship_size {
            let position_to_add = BoardPosition {
                x_position: match placement_direction {
                    PlacementDirection::NORTH | PlacementDirection::SOUTH => {
                        start_position.x_position
                    }
                    PlacementDirection::EAST => start_position.x_position + index,
                    PlacementDirection::WEST => start_position.x_position - index,
                },
                y_position: match placement_direction {
                    PlacementDirection::NORTH => start_position.y_position - index,
                    PlacementDirection::SOUTH => start_position.y_position + index,
                    PlacementDirection::EAST | PlacementDirection::WEST => {
                        start_position.y_position
                    }
                },
            };

            positions.push(position_to_add)
        }

        eprintln!("Positions based on start ({start_position:?}): {positions:#?}");

        for position in &positions {
            if self.has_ship_at_cell(position) {
                return Err("There already is a ship on the path of this ship".to_string());
            }
        }

        return Ok(positions);
    }

    pub fn print_board(&self) {
        let allowed_chars = BoardPosition::get_allowed_characters();

        println!("  | {} |", { allowed_chars.join(" | ") });
        println!("----------------------------");

        for y_pos in 1..self.y_size + 1 {
            print!("{y_pos} |");
            for x_pos in 1..self.x_size + 1 {
                let pos_to_check = BoardPosition {
                    x_position: x_pos,
                    y_position: y_pos,
                };
                if self.has_ship_at_cell(&pos_to_check) {
                    print!(" 0");
                } else {
                    print!(" X");
                }

                print!(" |");
            }
            println!()
        }
    }

    pub fn has_ship_at_cell(&self, board_position: &BoardPosition) -> bool {
        return self.player_ships.iter().any(|ship| {
            ship.coordinates.iter().any(|pos| {
                pos.x_position == board_position.x_position
                    && pos.y_position == board_position.y_position
            })
        });
    }
}
