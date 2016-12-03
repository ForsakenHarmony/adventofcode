#![allow(dead_code)]

#[derive(Debug)]
enum Direction {
  North,
  South,
  East,
  West
}

#[derive(Debug)]
struct Runner<'a> {
  pub facing: Direction,
  pub position: Vec<i32>,
  pub steps: Vec<&'a str>
}

impl<'a> Runner<'a> {
  pub fn new(steps: Vec<&str>) -> Runner {
    Runner {
      facing: Direction::North,
      position: vec![0, 0],
      steps: steps.clone()
    }
  }
  
  fn turn(&mut self, right: bool) {
    use self::Direction::*;
    
    let new_facing = match self.facing {
      North => if right { East } else { West },
      East => if right { South } else { North },
      South => if right { West } else { East },
      West => if right { North } else { South }
    };
    
    self.facing = new_facing;
  }
  
  fn move_forward(&mut self, number: i32) {
    use self::Direction::*;
    
    let direction = match self.facing {
      North => vec!(0, 1),
      East => vec!(1, 0),
      South => vec!(0, -1),
      West => vec!(-1, 0)
    };
    
    self.position[0] += direction[0] * number;
    self.position[1] += direction[1] * number;
  }
  
  pub fn go(&mut self) {
    //    println!("START pos: {:?}, facing: {:?}", self.position, self.facing);
    self.position = vec![0, 0];
    self.facing = Direction::North;
    
    for step in self.steps.clone() {
      let curr_step: (&str, &str) = step.split_at(1);
      
      let dir = curr_step.0;
      
      match dir {
        "R" => self.turn(true),
        "L" => self.turn(false),
        _ => {
          println!("Error: Invalid input");
          break;
        }
      }
      
      let number = curr_step.1.parse::<i32>().unwrap();
      
      self.move_forward(number);
      
      //      println!("step: {:?}, pos: {:?}, facing: {:?}", curr_step,
      //               self.position, self.facing);
    }
    
    //    println!("END pos: {:?}, facing: {:?}", self.position, self.facing);
  }
  
  pub fn visited_twice(&mut self) -> Vec<i32> {
    self.position = vec![0, 0];
    self.facing = Direction::North;
    
    let mut visited: Vec<Vec<i32>> = vec!(vec!(0, 0));
    let mut visited_again_count = 0;
    
    'outer: for step in self.steps.clone() {
      let curr_step: (&str, &str) = step.split_at(1);
      let dir = curr_step.0;
      
      println!("{:?} -> facing: {:?}, step: {:?}", self.position, self.facing,
               step);
      
      match dir {
        "R" => self.turn(true),
        "L" => self.turn(false),
        _ => {
          println!("Error: Invalid input");
          break;
        }
      }
      
      let number = curr_step.1.parse::<i32>().unwrap();
      
      for i in 0..number {
        self.move_forward(1);
        
        let res: Option<usize>;
        
        {
          res = visited.iter().position(|r| *r == self.position);
          
          println!("{:?} -> {:?}", self.position, res);
        }
        
        if res != None {
          visited_again_count += 1;
          
          println!("{:?} -> again: {:?}, index: {:?}",
                   self.position, visited[res.unwrap()], res.unwrap());
          
          if visited_again_count == 2 {
            break 'outer;
          }
        }
        
        visited.push(self.position.clone());
      }
      
      println!("{:?} -> facing: {:?} \n", self.position, self.facing);
    }
    
    self.position.clone()
  }
  
  pub fn get_distance(&self) -> i32 {
    self.position[0].abs() + self.position[1].abs()
  }
}

pub fn find_distance(input: &str) -> i32 {
  let split: Vec<&str> = input.split(", ").collect();
  
  let mut runner = Runner::new(split);
  runner.go();
  runner.get_distance()
}

pub fn dist_to_second_visit(input: &str) -> i32 {
  let split: Vec<&str> = input.split(", ").collect();
  
  let mut runner = Runner::new(split);
  runner.visited_twice();
  runner.get_distance()
}
