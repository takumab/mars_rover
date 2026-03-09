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

    pub fn execute(&self, _commands: &str) -> String {
        String::from("0 0 N")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_have_initial_coordinates_of_0_0_facing_north() {
        let rover = Rover::new();
        let result = rover.execute("");
        assert_eq!(result, "0 0 N");
    }
}
