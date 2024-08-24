use super::coords::Coords;
use super::directions::Directions;

#[derive(Debug)]
pub struct Snake {
    // The position of each block making up the body of snake
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
        let mut new_positions: [Coords; 20] = [Coords { x: -1, y: -1 }; 20];

        for (i, c) in self.positions.iter().enumerate() {
            // Grab the current head position and increment its position into new_positions.
            if i == 0 {
                new_positions[0] = current_head;

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
        }

        self.positions = new_positions;

        self
    }
}

mod tests {
    use super::*;

    //
    // Tests a new, baby snake's forward steps.
    //
    #[test]
    fn baby_snake_step() {
        let mut  snake = Snake { 
            positions: [Coords { x: -1, y: -1}; 20],
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

    #[test]
    //
    // Tests a teenager (multiple blocks) snake's forward steps.
    //
    fn teenager_snake_step_up() {
        let mut  snake = Snake { 
            positions: [Coords { x: -1, y: -1}; 20],
            direction: Directions::Up,
        };

        for i in 0..13 {
            snake.positions[i].x = 20;
            snake.positions[i].y = i as i16 + 10;
        }

        snake.step();

        // The head should have moved one step up
        assert_eq!(snake.positions[0].x, 20);
        assert_eq!(snake.positions[0].y, 9);

        // Check that the elements in the array have effectively all shifted down 1
        for i in 12..13 {
            assert_eq!(snake.positions[i].x, 20);
            assert_eq!(snake.positions[i].y, i as i16 + 9);
        }

    }
}