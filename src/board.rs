use unicode_segmentation::UnicodeSegmentation;

pub struct Board {
    pub x_size: u16,
    pub y_size: u16,
}

#[derive(Debug)]
pub struct BoardPosition {
    pub x_position: u16,
    pub y_position: u16,
}

impl BoardPosition {
    /// Generates a [BoardPosition] from a given string. The input is expected to be a character
    /// from the alphabet (e.g. 'C') concatenated with an integer (e.g. 10): C10.
    pub fn from_string(position_string: &str) -> Option<BoardPosition> {
        if position_string.graphemes(true).count() < 2 {
            return None;
        }

        let allowed_chars = ["A", "B", "C", "D", "E", "F", "G"];

        let first_character = &position_string[..1];

        // Parse first character
        let x_pos: u16 = match allowed_chars.iter().position(|&c| c == first_character) {
            None => {
                eprintln!("The character '{first_character} was not a valid character!'");
                return None;
            }
            Some(index) => index,
        } as u16;

        // Parse second character
        let second_character = &position_string[1..];

        eprintln!("Second character: {second_character}");

        let Ok(y_pos) = second_character.parse::<u16>() else {
            eprintln!("Could not parse '{second_character} into an integer!'");
            return None;
        };

        return Some(BoardPosition {
            x_position: x_pos,
            y_position: y_pos,
        });
    }
}

impl Board {
    pub fn is_position_on_board(&self, position: &BoardPosition) -> Result<(), &str> {
        return if position.x_position >= self.x_size || position.x_position < 0 {
            Err("X position is out of bounds")
        } else if position.y_position >= self.y_size || position.y_position < 0 {
            Err("Y position is out of bounds")
        } else {
            Ok(())
        };
    }
}
