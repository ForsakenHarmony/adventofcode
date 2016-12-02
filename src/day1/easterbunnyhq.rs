enum Direction {
  North,
  South,
  East,
  West
}

struct Runner {
  pub facing: Direction,
  pub position: Vec<i32>,
  pub steps: Vec<&'static str>
}

impl Runner {
  pub fn new(steps: Vec<&str>) -> Runner {
    Runner{
      facing: Direction::North,
      position: vec!(0,0),
      steps: steps
    }
  }
}

pub fn find_pos(input: &str) -> Vec<i32> {
  let split: Vec<&'static str> = input.split(", ").collect();
  
  
  
  let mut runner = Runner::new(split);
  
  turn(runner, true);
  
  println!("{:?}", split);
  
  vec!(0, 0)
}

fn turn(r: Runner, right: bool) {
  let new_facing = match r.facing {
    Direction::North => if right { Direction::East } else { Direction::West },
    Direction::East => if right { Direction::South } else { Direction::North },
    Direction::South => if right { Direction::West } else { Direction::East },
    Direction::West => if right { Direction::North } else { Direction::South }
  };
}
