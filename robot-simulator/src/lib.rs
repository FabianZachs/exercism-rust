// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
enum Instruction {
    Left,
    Right,
    Advance,
}

struct Position {
    x: i32,
    y: i32,
}

pub struct Robot {
    position: Position,
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot {
            position: Position { x, y },
            direction: d,
        }
    }

    pub fn turn_right(self) -> Self {
        let direction = match self.direction {
            Direction::West => Direction::North,
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
        };

        Robot { direction, ..self }
    }

    pub fn turn_left(self) -> Self {
        let direction = match self.direction {
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            Direction::North => Direction::West,
        };

        Robot { direction, ..self }
    }

    pub fn advance(self) -> Self {
        let mut new_robot = Robot { ..self };
        match new_robot.direction {
            Direction::West => new_robot.position.x -= 1,
            Direction::North => new_robot.position.y += 1,
            Direction::East => new_robot.position.x += 1,
            Direction::South => new_robot.position.y -= 1,
        };

        new_robot
    }

    pub fn instructions(self, instructions: &str) -> Self {
        //let mut new_robot = Robot { ..self };
        //for c in instructions.chars() {
        //    new_robot = match c {
        //        'R' => new_robot.turn_right(),
        //        'L' => new_robot.turn_left(),
        //        'A' => new_robot.advance(),
        //        _ => panic!("Invalid instruction"),
        //    };
        //}
        instructions.chars().fold(self, |robot, c| match c {
            'R' => robot.turn_right(),
            'L' => robot.turn_left(),
            'A' => robot.advance(),
            _ => panic!("Invalid instruction"),
        })
    }

    pub fn position(&self) -> (i32, i32) {
        let Position { x, y } = self.position;

        (x, y)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
