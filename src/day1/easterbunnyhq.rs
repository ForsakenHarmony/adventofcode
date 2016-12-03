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
  
  pub fn turn(&mut self, right: bool) {
    use self::Direction::*;
    
    let new_facing = match self.facing {
      North => if right { East } else { West },
      East => if right { South } else { North },
      South => if right { West } else { East },
      West => if right { North } else { South }
    };
    
    self.facing = new_facing;
  }
  
  pub fn move_forward(&mut self, number: i32) {
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
    println!("START pos: {:?}, facing: {:?}", self.position, self.facing);
    
    while self.steps.len() > 0 {
      let curr_element = self.steps.pop().unwrap();
      let curr_step: (&str, &str) = curr_element.split_at(1);
      
      let dir = curr_step.0;
      
      if dir.eq("R") {
        self.turn(true);
      } else if dir.eq("L") {
        self.turn(false);
      } else {
        println!("Error: Invalid input");
        break;
      }
  
      let number = curr_step.1.parse::<i32>().unwrap();
      
      self.move_forward(number);
      
//      let string_thing = "test";
//
      println!("step: {:?}, pos: {:?}, facing: {:?}", curr_element,
               self.position, self.facing);
    }
  
    println!("END pos: {:?}, facing: {:?}", self.position, self.facing);
  }
  
  pub fn get_distance(&self) -> i32 {
    self.position[0] + self.position[1]
  }
}

pub fn find_distance(input: &str) -> i32 {
  let split: Vec<&str> = input.split(", ").collect();
  
  let mut runner = Runner::new(split);
  
  runner.go();
  
//  println!("{:?}", runner);
  
  runner.get_distance()
}


