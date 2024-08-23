use super::coords::Coords;
use super::directions::Directions;

#[derive(Debug)]
pub struct Snake {
    // The position of each block making up the body of snake
    // and [0] being the head
    // The idea is that when snake is moving e.g. left, block 0
    // Y would be decreasing on each tick.
    // This array should be looped through on each tick so that 
    // we can update all part of snakes body according to the current direction.
    // Snake would want to look like he's moving in that direction, and so
    // on each tick we would need to remove the last element, add a new element
    // to the top of the array which would be in the position the head has moved to
    pub positions: [Coords; 20],

    // Holds the direction snake's head is currently facing
    pub direction: Directions,
}

impl Snake {
    pub fn step(&mut self) -> &mut Snake {
        // if snake is 1, just move forward - erase old block, write new block
        // if he is > 1, pop bit off tail and push on top in the direction you're going

        match self.direction {
            Directions::Up => self.positions[0].y -= 1,
            Directions::Down => self.positions[0].y += 1,
            Directions::Left => self.positions[0].x -= 1,
            Directions::Right => self.positions[0].x += 1,
            _ => {}
        }

        // for p in self.positions.iter() {
        //     match p.x > -1 && p.y > -1 => 
        // }

        

        self
    }
}