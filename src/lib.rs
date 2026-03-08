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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RoverError {
    InvalidCommand,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Position {
    pub point: Point,
    pub direction: Direction,
}

type Commands = Vec<char>;
pub fn rover_controller(
    commands: Commands,
    direction: Direction,
) -> Result<Position, RoverError> {
    let mut point = Point { x: 0, y: 0 };
    for command in commands {
       point = move_forward(&point, direction);
    }

    Ok(Position {
        point,
        direction,
    })
}

fn move_forward(point: &Point, direction: Direction) -> Point {
    match direction {
        Direction::North => Point { x: point.x, y: point.y + 1 },
        Direction::East=> Point { x: point.x + 1, y: point.y },
        Direction::West=> Point { x: point.x - 1, y: point.y },
        Direction::South => Point { x: point.x, y: point.y - 1 },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_move_forward_from_0_0_facing_north() {
        let commands = vec!['M'];
        let starting_direction = Direction::North;

        let expected_final_position = Position {
            point: Point { x: 0, y: 1 },
            direction: Direction::North,
        };

        let result = rover_controller(commands, starting_direction);

        assert_eq!(result, Ok(expected_final_position));
    }

    #[test]
    fn should_move_forward_twice_from_0_0_facing_north() {
        let commands = vec!['M', 'M'];
        let starting_direction = Direction::North;

        let expected_final_position = Position {
            point: Point { x: 0, y: 2 },
            direction: Direction::North,
        };

        let result = rover_controller(commands, starting_direction);

        assert_eq!(result, Ok(expected_final_position));
    }

    #[test]
    fn should_move_forward_thrice_from_0_0_facing_north() {
        let commands = vec!['M', 'M', 'M'];
        let starting_direction = Direction::North;

        let expected_final_position = Position {
            point: Point { x: 0, y: 3 },
            direction: Direction::North,
        };

        let result = rover_controller(commands, starting_direction);

        assert_eq!(result, Ok(expected_final_position));
    }

    #[test]
    fn should_move_forward_from_0_0_facing_east() {
        let commands = vec!['M'];
        let starting_direction = Direction::East;

        let expected_final_position = Position {
            point: Point { x: 1, y: 0 },
            direction: Direction::East,
        };

        let result = rover_controller(commands, starting_direction);

        assert_eq!(result, Ok(expected_final_position));
    }

    #[test]
    fn should_move_forward_twice_from_0_0_facing_east() {
        let commands = vec!['M', 'M'];
        let starting_direction = Direction::East;

        let expected_final_position = Position {
            point: Point { x: 2, y: 0 },
            direction: Direction::East,
        };

        let result = rover_controller(commands, starting_direction);

        assert_eq!(result, Ok(expected_final_position));
    }

    #[test]
    fn should_move_forward_once_from_0_0_facing_west() {
        let commands = vec!['M'];
        let starting_direction = Direction::West;

        let expected_final_position = Position {
            point: Point { x: -1, y: 0 },
            direction: Direction::West,
        };

        let result = rover_controller(commands, starting_direction);

        assert_eq!(result, Ok(expected_final_position));
    }

    #[test]
    fn should_move_forward_twice_from_0_0_facing_west() {
        let commands = vec!['M', 'M'];
        let starting_direction = Direction::West;

        let expected_final_position = Position {
            point: Point { x: -2, y: 0 },
            direction: Direction::West,
        };

        let result = rover_controller(commands, starting_direction);

        assert_eq!(result, Ok(expected_final_position));
    }

    #[test]
    fn should_move_forward_once_from_0_0_facing_south() {
        let commands = vec!['M'];
        let starting_direction = Direction::South;

        let expected_final_position = Position {
            point: Point { x: 0, y: -1 },
            direction: Direction::South,
        };

        let result = rover_controller(commands, starting_direction);

        assert_eq!(result, Ok(expected_final_position));
    }
}
