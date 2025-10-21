// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    d: Direction,
    x: i32,
    y: i32,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self { d, x, y }
    }

    #[must_use]
    pub fn turn_right(mut self) -> Self {
        self.d = match self.d {
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::North => Direction::East,
        };
        self
    }

    #[must_use]
    pub fn turn_left(mut self) -> Self {
        self.d = match self.d {
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
            Direction::North => Direction::West,
        };
        self
    }

    #[must_use]
    pub fn advance(mut self) -> Self {
        match self.d {
            Direction::East => self.x += 1,
            Direction::South => self.y -= 1,
            Direction::West => self.x -= 1,
            Direction::North => self.y += 1,
        };
        self
    }

    #[must_use]
    pub fn instructions(mut self, instructions: &str) -> Self {
        for b in instructions.as_bytes() {
            self = match *b {
                b'L' => self.turn_left(),
                b'R' => self.turn_right(),
                b'A' => self.advance(),
                _ => panic!("invalid instruction"),
            };
        };
        self
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
