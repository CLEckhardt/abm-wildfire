/*
WILDFIRE
*/


/*
NOTES

- Entity ID for trees starts at 0 in upper left and increments starting going across,
  working down row-wise
*/

use std::{thread, time, io};
use rand::distributions::{Bernoulli, Distribution};


const CYCLE_TIME: u64 = 250;  // in ms
const FOREST_X: usize = 90;
const FOREST_Y: usize = 30;


type EntityId = usize;
type ArPosition = [Position; FOREST_X * FOREST_Y];
type ArState = [TreeState; FOREST_X * FOREST_Y];


#[derive(Clone, Copy, PartialEq)]
struct Position(usize, usize);

impl Position {
    fn entity_id(&self) -> EntityId {
        self.0 + self.1 * FOREST_X
    }

    fn up(&self) -> Option<Position> {
        if self.1 == 0 {
            None
        } else {
            Some(Position(self.0, self.1 - 1))
        }
    }

    fn down(&self) -> Option<Position> {
        if self.1 == FOREST_Y - 1 {
            None
        } else {
            Some(Position(self.0, self.1 + 1))
        }
    }

    fn right(&self) -> Option<Position> {
        if self.0 == FOREST_X - 1 {
            None
        } else {
            Some(Position(self.0 + 1, self.1))
        }
    }

    fn left(&self) -> Option<Position> {
        if self.0 == 0 {
            None
        } else {
            Some(Position(self.0 - 1, self.1))
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum TreeState {
    Clear,
    Living,
    Ignited,
    Burning,
    Burned,
}

impl TreeState {
    fn ignite_living(&mut self) {
        match &self {
            TreeState::Living => *self = TreeState::Ignited,
            _ => (),
        } 
    }
}

// COMPONENTS


struct Trees {
    position: ArPosition,
    state: ArState,
}

impl Trees {
    fn init() -> Trees {
        Trees {
            position: [Position(0,0); FOREST_X * FOREST_Y],
            state: [TreeState::Clear; FOREST_X * FOREST_Y],
        }
    }
}


// SYSTEMS

fn initialize_forest(density: f64) -> Trees {
    let bd = Bernoulli::new(density).unwrap();
    let mut rng = rand::thread_rng();

    let mut forest = Trees::init();
    let mut p: usize = 0;
    for j in 0..FOREST_Y {
        for i in 0..FOREST_X {
            forest.position[p] = Position(i, j);
            if bd.sample(&mut rng) {
                forest.state[p] = TreeState::Living;
            }
            p += 1;
        }
    }
    forest
}


fn paint_forest(forest: &Trees) {
    let mut out = String::new();
    for i in 0..(FOREST_X * FOREST_Y) {
        if i % FOREST_X == 0 {
            out.push('|'); // border character
        }
        match forest.state[i] {
            TreeState::Clear => out.push(' '),
            TreeState::Living => out.push('T'),
            TreeState::Ignited => out.push('#'),
            TreeState::Burning => out.push('%'),
            TreeState::Burned => out.push('X'),
        }
        if i % FOREST_X == (FOREST_X - 1) {
            out.push('|');
            out.push('\n');
        }
    }
    print!("\x1B[2J\x1B[1;1H");
    print!("{}", out);
}

fn start_fire(forest: &mut Trees) {
    for i in 0..(FOREST_X * FOREST_Y) {
        if (forest.position[i].0 != 0) | (forest.state[i] == TreeState::Clear) {
            continue;
        }
        forest.state[i] = TreeState::Ignited;
    }
}

fn spread_fire(forest: &mut Trees) {
    for i in 0..(FOREST_X * FOREST_Y) {
        match forest.state[i] {
            TreeState::Burning => {
                forest.state[i] = TreeState::Burned
            },
            TreeState::Ignited => {
                forest.state[i] = TreeState::Burning
            },
            _ => (),
        }
    }

    for i in 0..(FOREST_X * FOREST_Y) {
        if forest.state[i] == TreeState::Burning {
            if let Some(neighbor_up) = forest.position[i].up() {
                forest.state[neighbor_up.entity_id()].ignite_living();
            }
            if let Some(neighbor_down) = forest.position[i].down() {
                forest.state[neighbor_down.entity_id()].ignite_living();
            }
            if let Some(neighbor_right) = forest.position[i].right() {
                forest.state[neighbor_right.entity_id()].ignite_living();
            }
            if let Some(neighbor_left) = forest.position[i].left() {
                forest.state[neighbor_left.entity_id()].ignite_living();
            }
        }
    }
}

fn check_fire_active(trees: &ArState) -> bool {
    trees.iter().any(|&x| x.eq(&TreeState::Ignited) | x.eq(&TreeState::Burning))
}

fn calculate_burn_rate(trees: &ArState) -> f64 {
    let burned = trees.iter().filter(|&x| x.eq(&TreeState::Burned)).count() as f64;
    let living = trees.iter().filter(|&x| x.eq(&TreeState::Living)).count() as f64;
    burned / (living + burned)
}


fn main() {

    println!("Enter forest density (decimal > 0.0 and <= 1.0):");
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read input");
    let input = buffer.trim();
    let density = input.parse::<f64>().expect("Input was not a decimal");
    if !( (density > 0.0) | (density <= 1.0) ) {
        panic!("Input must be greater than 0 and less than 1.0")
    }

    let mut forest = initialize_forest(density);
    paint_forest(&forest);
    thread::sleep(time::Duration::from_millis(1500));

    start_fire(&mut forest);
    paint_forest(&forest);

    for _ in 0..1024 {
        thread::sleep(time::Duration::from_millis(CYCLE_TIME));

        spread_fire(&mut forest);
        paint_forest(&forest);
        if !check_fire_active(&forest.state) {
            break;
        }
    }
    println!("");
    println!("% forest burned: {}", calculate_burn_rate(&forest.state));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_neighbor_up() {
        assert!(Position(0, 0).up().is_none());
        assert!(match Position(1, 1).up() {
            Some(x) => x.eq(&Position(1, 0)),
            None => false,
        });
    }

    #[test]
    fn find_neighbor_down() {
        assert!(Position(0, FOREST_Y - 1).down().is_none());
        assert!(match Position(1, 1).down() {
            Some(x) => x.eq(&Position(1, 2)),
            None => false,
        });
    }

    #[test]
    fn find_neighbor_right() {
        assert!(Position(FOREST_X - 1, 0).right().is_none());
        assert!(match Position(1, 1).right() {
            Some(x) => x.eq(&Position(2, 1)),
            None => false,
        });
    }

    #[test]
    fn find_neighbor_left() {
        assert!(Position(0, 0).left().is_none());
        assert!(match Position(1, 1).left() {
            Some(x) => x.eq(&Position(0, 1)),
            None => false,
        });
    }

}
