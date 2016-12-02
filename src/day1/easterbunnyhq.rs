enum Direction {
  North,
  South,
  East,
  West
}

struct Runner {
  facing: Direction,
  position: Vec<i32>
}

pub fn find_pos(input: &str) -> Vec<i32> {
  let split: Vec<&str> = input.split(", ").collect();
  
  println!("{:?}", split);
  
  vec!(0, 0)
}

pub fn turn(r: &Runner, right: bool) {
  let new_facing = match r.facing {
    Direction::North => if right {Direction::East} else {Direction::West}
    Direction::East => if right {Direction::South} else {Direction::North}
    Direction::South => if right {Direction::West} else {Direction::East}
    Direction::West => if right {Direction::North} else {Direction::South}
  };
}
