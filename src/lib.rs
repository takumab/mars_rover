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


#[cfg(test)]
mod tests {

    #[test]
    fn it_smoke_test() {
        assert_eq!(2+2, 4);
    }
}
