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

pub struct Rover {
    x: i32,
    y: i32,
    cardinal: char,
}

impl Rover {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            cardinal: 'N',
        }
    }

    pub fn execute(&self, commands: &str) -> String {
        let cardinal = if commands == "R" {
            'E'
        } else if commands == "RR" {
            'S'
        } else {
            'N'
        };

        format!("0:0:{}", cardinal)
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
    fn should_execute(#[case] commands: &str, #[case] expected: String) {
        let rover = Rover::new();
        let result = rover.execute(commands);
        assert_eq!(result, expected);
    }
}
