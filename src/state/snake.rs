use std::error::Error;

use crate::engine::{graphics::Graphics, unicode::Unicode};

use super::{directions::Directions, snake_coords::SnakeCoords};

#[derive(Debug)]
pub struct Snake {
    // The position of each block making up the body of snake
    // Todo: this should not be a set size. When launching the
    // game, or eating, we need to create a new array the size
    // the snake is going to be, populate that, then use that as
    // the snake.
    pub positions: [SnakeCoords; 20],

    // Holds the direction snake's head is currently facing
    pub direction: Directions,

    // Snake is dead x_x
    pub x_x: bool,
}

impl Snake {
    pub fn new() -> Self {
        Snake {
            positions: [SnakeCoords::default(); 20],
            direction: Directions::None,
            x_x: false,
        }
    }

    //
    // Steps snake one block in the direction he is facing.
    // Effectively this code is removing the tail block, shifting all elements
    // in the array down one and adds a new head (0) block at the shifted
    // coordinates. This algorithm will be used to draw the snake on every
    // tick to make it look like it's moving.
    //
    pub fn step(&mut self) -> &mut Snake {
        let current_head = self.positions[0];
        let mut new_positions: [SnakeCoords; 20] = [SnakeCoords::default(); 20];

        for (i, c) in self.positions.iter().enumerate() {
            // Grab the current head position and increment its position into new_positions.
            if i == 0 {
                new_positions[0] = current_head;
                new_positions[0].facing = self.direction;

                match self.direction {
                    Directions::Up => {
                        new_positions[0].coords.y -= 1;
                    }
                    Directions::Down => {
                        new_positions[0].coords.y += 1;
                    }
                    Directions::Left => {
                        new_positions[0].coords.x -= 1;
                    }
                    Directions::Right => {
                        new_positions[0].coords.x += 1;
                    }
                    _ => {}
                }

                continue;
            }

            // Don't process snake's inactive blocks.
            if !c.coords.is_active() {
                continue;
            }

            new_positions[i].coords.x = self.positions[i - 1].coords.x;
            new_positions[i].coords.y = self.positions[i - 1].coords.y;
            new_positions[i].facing = self.positions[i - 1].facing;
        }

        self.positions = new_positions;

        self
    }

    //
    // Grow snake by the given number of blocks (amount).
    // Todo: when Snake grows, really he should grow the positions array. Currently it's fixed.
    //
    pub fn grow(&mut self, amount: usize) -> &mut Snake {
        let mut positions = self.positions;

        for (i, p) in self.positions.iter().enumerate() {
            // Skip if the previous position did not have valid coords but
            // the current does. This means we are at the tail.
            if !p.coords.is_active() && positions[i - 1].coords.is_active() {
                for j in 0..amount {
                    // Read the values of the tail so we know what the oritentation should be
                    // for the new one.
                    let previous_tail = positions[j + 1 - 1];

                    let mut new_position = SnakeCoords::new(1, 1, previous_tail.facing);

                    match previous_tail.facing {
                        Directions::Up => {
                            new_position.coords.x = previous_tail.coords.x;
                            new_position.coords.y = previous_tail.coords.y + 1;
                        }
                        Directions::Down => {
                            new_position.coords.x = previous_tail.coords.x;
                            new_position.coords.y = previous_tail.coords.y - 1;
                        }
                        Directions::Left => {
                            new_position.coords.x = previous_tail.coords.x + 1;
                            new_position.coords.y = previous_tail.coords.y;
                        }
                        Directions::Right => {
                            new_position.coords.x = previous_tail.coords.x - 1;
                            new_position.coords.y = previous_tail.coords.y;
                        }
                        Directions::None => {}
                    }

                    positions[j + i] = new_position;
                }

                break;
            }
        }

        self.positions = positions;
        self
    }

    //
    // Loops through each snake position and draws a space to clear it.
    //
    pub fn clear(&self) -> Result<(), Box<dyn Error>> {
        for (i, p) in self.positions.iter().enumerate() {
            // Clear all positions that don't have a facing or have and invalid position.
            // Always clear i when i is 0 as we want to make sure the starting piece is cleared.
            if p.facing == Directions::None && !p.coords.is_active() && i > 0 {
                continue;
            }

            let (ux, uy) = p.coords.to_unsigned_tuple();

            Graphics::draw_char(ux, uy, Unicode::Space)?;
        }

        Ok(())
    }

    //
    // Returns the number of active (x & y > -1, with a direction) blocks in Snake.
    //
    pub fn active_length(&self) -> Option<usize> {
        let active_pos = self
            .positions
            .iter()
            .filter(|p| p.coords.is_active() && p.facing != Directions::None)
            .count();
        match active_pos {
            1.. => Some(active_pos),
            0 => None,
        }
    }

    //
    // Returns true if Snake has at least 1 active block.
    //
    pub fn is_active(&self) -> bool {
        self.active_length().is_some()
    }

    //
    // Returns true if Snake's head block has hit any part of his body.
    //
    pub fn has_hit_self(&self) -> bool {
        // Skip the head piece, otherwise this always returns true.
        let has_match = &self.positions[1..]
            .iter()
            .position(|p| p.coords == self.positions[0].coords);

        has_match.is_some()
    }
}

impl Default for Snake {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::engine::coords::Coords;

    use super::*;

    // Todo: test steps + facing changes (turns)

    //
    // Tests a new, baby snake's forward steps.
    //
    #[test]
    fn baby_snake_step() {
        let mut snake = Snake {
            positions: [SnakeCoords::default(); 20],
            direction: Directions::Up,
            x_x: false,
        };

        snake.positions[0].coords.x = 5;
        snake.positions[0].coords.y = 5;

        snake.step();

        // Make sure only the head block has moved position.
        // Move is Up, which means up one line and so y decreases.
        assert_eq!(snake.positions[0].coords.x, 5);
        assert_eq!(snake.positions[0].coords.y, 4);

        // The rest of the body should be inactive.
        for i in 2..snake.positions.len() {
            assert_eq!(snake.positions[i].coords.x, -1);
            assert_eq!(snake.positions[i].coords.y, -1);
        }
    }

    //
    // Tests a teenager (multiple blocks) snake's forward steps.
    //
    #[test]
    fn teenager_snake_step_up() {
        let mut snake = Snake {
            positions: [SnakeCoords::default(); 20],
            direction: Directions::Up,
            x_x: false,
        };

        for i in 0..13 {
            snake.positions[i].coords.x = 20;
            snake.positions[i].coords.y = (i as i16) + 10;
        }

        // All snake positions after 13 should be inactive.
        for i in 13..snake.positions.len() {
            assert_eq!(snake.positions[i].coords.y, -1);
            assert_eq!(snake.positions[i].coords.x, -1);
        }

        snake.step();

        // The head should have moved one step up.
        assert_eq!(snake.positions[0].coords.x, 20);
        assert_eq!(snake.positions[0].coords.y, 9);

        // Check that the elements in the array have effectively all shifted down 1.
        for i in 2..13 {
            assert_eq!(snake.positions[i].coords.x, 20);
            assert_eq!(snake.positions[i].coords.y, (i as i16) + 9);
        }

        // All snake positions after 13 should be inactive.
        for i in 13..snake.positions.len() {
            assert_eq!(snake.positions[i].coords.y, -1);
            assert_eq!(snake.positions[i].coords.x, -1);
        }
    }

    //
    // Set up a snake and grow it a single block.
    //
    fn set_snake_and_grow(direction: Directions, grow_by: usize) -> Snake {
        let mut snake = Snake {
            positions: [SnakeCoords::new(-1, -1, direction); 20],
            direction: direction,
            x_x: false,
        };

        snake.positions[0].coords.x = 10;
        snake.positions[0].coords.y = 10;

        snake.grow(grow_by);

        snake
    }

    //
    // Check new snake position is in the right place when tail is facing up.
    //
    #[test]
    fn grow_amount_1_should_add_new_block_direction_up() {
        let snake = set_snake_and_grow(Directions::Up, 1);

        assert_eq!(snake.positions[0].coords.x, 10);
        assert_eq!(snake.positions[0].coords.y, 10);

        assert_eq!(snake.positions[1].coords.x, 10);
        assert_eq!(snake.positions[1].coords.y, 11);

        assert_eq!(snake.positions[2].coords.x, -1);
        assert_eq!(snake.positions[2].coords.y, -1);
    }

    //
    // Check new snake position is in the right place when tail is facing down.
    //
    #[test]
    fn grow_amount_1_should_add_new_block_direction_down() {
        let snake = set_snake_and_grow(Directions::Down, 1);

        assert_eq!(snake.positions[0].coords.x, 10);
        assert_eq!(snake.positions[0].coords.y, 10);

        assert_eq!(snake.positions[1].coords.x, 10);
        assert_eq!(snake.positions[1].coords.y, 9);

        assert_eq!(snake.positions[2].coords.x, -1);
        assert_eq!(snake.positions[2].coords.y, -1);
    }

    //
    // Check new snake position is in the right place when tail is facing left.
    //
    #[test]
    fn grow_amount_1_should_add_new_block_direction_left() {
        let snake = set_snake_and_grow(Directions::Left, 1);

        assert_eq!(snake.positions[0].coords.x, 10);
        assert_eq!(snake.positions[0].coords.y, 10);

        assert_eq!(snake.positions[1].coords.x, 11);
        assert_eq!(snake.positions[1].coords.y, 10);

        assert_eq!(snake.positions[2].coords.x, -1);
        assert_eq!(snake.positions[2].coords.y, -1);
    }

    //
    // Check new snake position is in the right place when tail is facing right.
    //
    #[test]
    fn grow_amount_1_should_add_new_block_direction_right() {
        let snake = set_snake_and_grow(Directions::Right, 1);

        assert_eq!(snake.positions[0].coords.x, 10);
        assert_eq!(snake.positions[0].coords.y, 10);

        assert_eq!(snake.positions[1].coords.x, 9);
        assert_eq!(snake.positions[1].coords.y, 10);

        assert_eq!(snake.positions[2].coords.x, -1);
        assert_eq!(snake.positions[2].coords.y, -1);
    }

    //
    // Check new snake position is in the right place when tail is facing right, more blocks.
    //
    #[test]
    fn grow_amount_6_should_add_new_block_direction_right() {
        let snake = set_snake_and_grow(Directions::Right, 6);

        assert_eq!(snake.positions[0].coords.x, 10);
        assert_eq!(snake.positions[0].coords.y, 10);

        assert_eq!(snake.positions[1].coords.x, 9);
        assert_eq!(snake.positions[1].coords.y, 10);

        assert_eq!(snake.positions[2].coords.x, 8);
        assert_eq!(snake.positions[2].coords.y, 10);

        assert_eq!(snake.positions[3].coords.x, 7);
        assert_eq!(snake.positions[4].coords.y, 10);

        assert_eq!(snake.positions[4].coords.x, 6);
        assert_eq!(snake.positions[4].coords.y, 10);

        assert_eq!(snake.positions[5].coords.x, 5);
        assert_eq!(snake.positions[5].coords.y, 10);

        assert_eq!(snake.positions[6].coords.x, 4);
        assert_eq!(snake.positions[6].coords.y, 10);

        assert_eq!(snake.positions[7].coords.x, -1);
        assert_eq!(snake.positions[7].coords.y, -1);
    }

    //
    // If the head block intersects any of the body, has_hit_self should return true.
    //
    #[test]
    fn has_hit_self_return_true() {
        let mut snake = Snake::new();
        snake.direction = Directions::Down;
        snake.positions[0].facing = Directions::Down;
        snake.positions[0].coords = Coords::new(5, 10);
        snake.grow(5);

        snake.positions[0].coords = Coords::new(5, 7);

        assert_eq!(true, Snake::has_hit_self(&snake));
    }

    //
    // Make sure has_hit_self returns false if the head does not intersect the body
    //
    #[test]
    fn has_hit_self_return_false() {
        let mut snake = Snake::new();
        snake.direction = Directions::Down;
        snake.positions[0].facing = Directions::Down;
        snake.positions[0].coords = Coords::new(5, 10);
        snake.grow(5);

        snake.positions[0].coords = Coords::new(5, 11);

        assert_eq!(false, Snake::has_hit_self(&snake));
    }
}
