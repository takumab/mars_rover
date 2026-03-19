// Mars Rover
// You’re part of the team that make Mars Rover. Develop
// the simulator program that takes commands and a map,
// they translate the commands and show the result position
// and direction of the Rover.

// Requirements
// Your program takes in input a rover‘s starting point (x, y) and the direction (N, S, E, W) it is facing
// a map describing the location of obstacles
// a list of commands to move and turn the rover (⬆️ : move forward, ➡️ : turn right 90°, ⬅️ : turn left 90°)
// When the rover encounters an obstacle, it does nothing.

// inputs = commands, starting point, direction
// actions = move forward, turn right, turn left
// outputs = final position and direction

use std::fmt::Display;

pub struct Rover {
    x: i32,
    y: i32,
    cardinal: Cardinal,
}
enum Cardinal {
    North,
    South,
    East,
    West,
}
impl Display for Cardinal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cardinal_str = match self {
            Cardinal::North => "N",
            Cardinal::South => "S",
            Cardinal::East => "E",
            Cardinal::West => "W",
        };
        write!(f, "{}", cardinal_str)
    }
}

impl Rover {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            cardinal: Cardinal::North,
        }
    }

    pub fn execute(&mut self, commands: &str) -> String {
        for command in commands.chars() {
            match command {
                'R' => {
                    self.cardinal = match self.cardinal {
                        Cardinal::North => Cardinal::East,
                        Cardinal::East => Cardinal::South,
                        Cardinal::South => Cardinal::West,
                        Cardinal::West => Cardinal::North,
                    }
                }
                'L' => {
                    self.cardinal = match self.cardinal {
                        Cardinal::North => Cardinal::West,
                        Cardinal::West => Cardinal::South,
                        Cardinal::South => Cardinal::East,
                        Cardinal::East => Cardinal::North,
                    }
                }
                'M' => {
                    match self.cardinal {
                        Cardinal::North => self.y += 1,
                        Cardinal::South => self.y -= 1,
                        Cardinal::East => self.x += 1,
                        Cardinal::West => self.x -= 1,
                    }
                }
                _ => todo!(),
            }
        }

        format!("{}:{}:{}", self.x, self.y, self.cardinal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::initial_coordinates_of_0_0_facing_north("", String::from("0:0:N"))]
    #[case::turn_right_to_cardinal_east("R", String::from("0:0:E"))]
    #[case::turn_right_to_cardinal_south("RR", String::from("0:0:S"))]
    #[case::turn_right_to_cardinal_west("RRR", String::from("0:0:W"))]
    #[case::turn_right_to_cardinal_north("RRRR", String::from("0:0:N"))]
    fn should_execute_right_rotation(#[case] commands: &str, #[case] expected: String) {
        let mut rover = Rover::new();
        let result = rover.execute(commands);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case::rotate_west("L", String::from("0:0:W"))]
    #[case::rotate_south("LL", String::from("0:0:S"))]
    #[case::rotate_east("LLL", String::from("0:0:E"))]
    #[case::rotate_north("LLLL", String::from("0:0:N"))]
    #[case::rotate_west("LLLLL", String::from("0:0:W"))]
    fn should_execute_left_rotation(#[case] commands: &str, #[case] expected: String) {
        let mut rover = Rover::new();
        let result = rover.execute(commands);
        assert_eq!(result, expected);
    }
    #[rstest]
    #[case::move_forward_once("M", String::from("0:1:N"))]
    #[case::move_forward_twice("MM", String::from("0:2:N"))]
    #[case::move_forward_thrice("MMM", String::from("0:3:N"))]
    #[case::move_forward_rotate_south("RRM", String::from("0:-1:S"))]
    #[case::move_forward_rotate_east("RM", String::from("1:0:E"))]
    #[case::move_forward_rotate_west("RRRM", String::from("-1:0:W"))]
    fn should_execute_move(#[case] commands: &str, #[case] expected: String) {
        let mut rover = Rover::new();
        let result = rover.execute(commands);
        assert_eq!(result, expected);
    }

    #[test]
    fn should_execute_moves_and_rotate() {
        let mut rover = Rover::new();
        let result = rover.execute("RMMLM");
        assert_eq!(result, String::from("2:1:N"));
    }

    #[test]
    fn should_have_final_position_of_3_2_s() {
        let mut rover = Rover::new();
        let result = rover.execute("RMMMLMMRR");
        assert_eq!(result, String::from("3:2:S"));
    }
}
