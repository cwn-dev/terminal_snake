use super::coords::Coords;
use super::directions::Directions;

#[derive(Debug)]
pub struct Snake {
    // The position of each block making up the body of snake
    // Todo: this should not be a set size. When launching the
    // game, or eating, we need to create a new array the size
    // the snake is going to be, populate that, then use that as
    // the snake.
    pub positions: [Coords; 20],

    // Holds the direction snake's head is currently facing
    pub direction: Directions,
}

impl Snake {
    //
    // Steps snake one block in the direction he is facing.
    // Effectively this code is removing the tail block, shifting all elements
    // in the array down one and adds a new head (0) block at the shifted
    // coordinates. This algorithm will be used to draw the snake on every
    // tick to make it look like it's moving.
    //
    pub fn step(&mut self) -> &mut Snake {
        let current_head = self.positions[0];
        let mut new_positions: [Coords; 20] = [Coords { x: -1, y: -1, facing: Directions::None }; 20];

        for (i, c) in self.positions.iter().enumerate() {
            // Grab the current head position and increment its position into new_positions.
            if i == 0 {
                new_positions[0] = current_head;
                new_positions[0].facing = self.direction;

                match self.direction {
                    Directions::Up => new_positions[0].y -= 1,
                    Directions::Down => new_positions[0].y += 1,
                    Directions::Left => new_positions[0].x -= 1,
                    Directions::Right => new_positions[0].x += 1,
                    _ => {}
                }
                
                continue;
            }

            // Don't process snake's inactive blocks.
            if c.x == -1 || c.y == -1 {
                continue;
            }

            // Is this the last piece of the active current snake? Throw it away
            // if c.x == -1 && c.y == -1 && self.positions[i - 1].x != -1 && self.positions[i - 1].y != -1
            // {
            //     break;
            // }

            new_positions[i].x = self.positions[i - 1].x;
            new_positions[i].y = self.positions[i - 1].y;
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

        for (i, p) in self.positions.iter().enumerate() { // todo: look into iter().enumerate()
            // Only continue if the previous x & y had values but the current does not.
            // This means we are at the tail.
            if p.x == -1 || p.x == -1 && positions[i - 1].x != -1 && positions[i - 1].y != -1 {
                for j in 0..amount {                
                    // Read the values of the tail so we know what the oritentation should be
                    // for the new one.
                    let previous_tail = positions[(j + 1) - 1];

                    let mut new_position = Coords { x: 1, y: 1, facing: previous_tail.facing };

                    match previous_tail.facing {
                        Directions::Up => {
                            new_position.x = previous_tail.x;
                            new_position.y = previous_tail.y + 1;
                        },
                        Directions::Down => {
                            new_position.x = previous_tail.x;
                            new_position.y = previous_tail.y - 1;
                        },
                        Directions::Left => {
                            new_position.x = previous_tail.x + 1;
                            new_position.y = previous_tail.y;
                        },
                        Directions::Right => {
                            new_position.x = previous_tail.x - 1;
                            new_position.y = previous_tail.y;
                        },
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
}

#[cfg(test)]
mod tests {
    use super::*;

    // Todo: test steps + facing changes (turns)

    //
    // Tests a new, baby snake's forward steps.
    //
    #[test]
    fn baby_snake_step() {
        let mut  snake = Snake { 
            positions: [Coords { x: -1, y: -1, facing: Directions::None }; 20],
            direction: Directions::Up,
        };

        snake.positions[0].x = 5;
        snake.positions[0].y = 5;

        snake.step();

        // Make sure only the head block has moved position.
        // Move is Up, which means up one line and so y decreases.
        assert_eq!(snake.positions[0].x, 5);
        assert_eq!(snake.positions[0].y, 4);
        
        // The rest of the body should be inactive.
        for i in 2..snake.positions.len() {
            assert_eq!(snake.positions[i].x, -1);
            assert_eq!(snake.positions[i].y, -1);
        }
    }

    //
    // Tests a teenager (multiple blocks) snake's forward steps.
    //
    #[test]
    fn teenager_snake_step_up() {
        let mut snake = Snake { 
            positions: [Coords { x: -1, y: -1, facing: Directions::None }; 20],
            direction: Directions::Up,
        };

        for i in 0..13 {
            snake.positions[i].x = 20;
            snake.positions[i].y = i as i16 + 10;
        }

        // All snake positions after 13 should be inactive.
        for i in 13..snake.positions.len() {
            assert_eq!(snake.positions[i].y, -1);
            assert_eq!(snake.positions[i].x, -1);
        }

        snake.step();

        // The head should have moved one step up.
        assert_eq!(snake.positions[0].x, 20);
        assert_eq!(snake.positions[0].y, 9);

        // Check that the elements in the array have effectively all shifted down 1.
        for i in 2..13 {
            assert_eq!(snake.positions[i].x, 20);
            assert_eq!(snake.positions[i].y, i as i16 + 9);
        }

        // All snake positions after 13 should be inactive.
        for i in 13..snake.positions.len() {
            assert_eq!(snake.positions[i].y, -1);
            assert_eq!(snake.positions[i].x, -1);
        }        
    }

    //
    // Set up a snake and grow it a single block.
    //
    fn set_snake_and_grow(direction: Directions, grow_by: usize) -> Snake {
        let mut snake = Snake { 
            positions: [Coords { x: -1, y: -1, facing: direction }; 20],
            direction: direction,
        };

        snake.positions[0].x = 10;
        snake.positions[0].y = 10;

        snake.grow(grow_by);

        snake
    }

    //
    // Check new snake position is in the right place when tail is facing up.
    //
    #[test]
    fn grow_amount_1_should_add_new_block_direction_up() {
        let snake = set_snake_and_grow(Directions::Up, 1);

        assert_eq!(snake.positions[0].x, 10);
        assert_eq!(snake.positions[0].y, 10);

        assert_eq!(snake.positions[1].x, 10);
        assert_eq!(snake.positions[1].y, 11);

        assert_eq!(snake.positions[2].x, -1);
        assert_eq!(snake.positions[2].y, -1);
    }

    //
    // Check new snake position is in the right place when tail is facing down.
    //
    #[test]
    fn grow_amount_1_should_add_new_block_direction_down() {
        let snake = set_snake_and_grow(Directions::Down, 1);

        assert_eq!(snake.positions[0].x, 10);
        assert_eq!(snake.positions[0].y, 10);

        assert_eq!(snake.positions[1].x, 10);
        assert_eq!(snake.positions[1].y, 9);

        assert_eq!(snake.positions[2].x, -1);
        assert_eq!(snake.positions[2].y, -1);
    }

    //
    // Check new snake position is in the right place when tail is facing left.
    //
    #[test]
    fn grow_amount_1_should_add_new_block_direction_left() {
        let snake = set_snake_and_grow(Directions::Left, 1);

        assert_eq!(snake.positions[0].x, 10);
        assert_eq!(snake.positions[0].y, 10);

        assert_eq!(snake.positions[1].x, 11);
        assert_eq!(snake.positions[1].y, 10);

        assert_eq!(snake.positions[2].x, -1);
        assert_eq!(snake.positions[2].y, -1);
    }

    //
    // Check new snake position is in the right place when tail is facing right.
    //
    #[test]
    fn grow_amount_1_should_add_new_block_direction_right() {
        let snake = set_snake_and_grow(Directions::Right, 1);

        assert_eq!(snake.positions[0].x, 10);
        assert_eq!(snake.positions[0].y, 10);

        assert_eq!(snake.positions[1].x, 9);
        assert_eq!(snake.positions[1].y, 10);

        assert_eq!(snake.positions[2].x, -1);
        assert_eq!(snake.positions[2].y, -1);
    }

    #[test]
    fn grow_amount_6_should_add_new_block_direction_right() {
        let snake = set_snake_and_grow(Directions::Right, 6);

        assert_eq!(snake.positions[0].x, 10);
        assert_eq!(snake.positions[0].y, 10);

        assert_eq!(snake.positions[1].x, 9);
        assert_eq!(snake.positions[1].y, 10);

        assert_eq!(snake.positions[2].x, 8);
        assert_eq!(snake.positions[2].y, 10);

        assert_eq!(snake.positions[3].x, 7);
        assert_eq!(snake.positions[4].y, 10);

        assert_eq!(snake.positions[4].x, 6);
        assert_eq!(snake.positions[4].y, 10);

        assert_eq!(snake.positions[5].x, 5);
        assert_eq!(snake.positions[5].y, 10);

        assert_eq!(snake.positions[6].x, 4);
        assert_eq!(snake.positions[6].y, 10);

        assert_eq!(snake.positions[7].x, -1);
        assert_eq!(snake.positions[7].y, -1);
    }
}