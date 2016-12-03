struct Runner<'a> {
  pub position: Vec<i32>,
  pub steps: Vec<Vec<&'a str>>
}

impl<'a> Runner<'a> {
  pub fn new(steps: Vec<Vec<&str>>) -> Runner {
    
  }
}

pub fn get_code(input: &str) -> str {
  let split: Vec<&str> = input.split("\n").collect();
  
  let split_split: Vec<Vec<&str>> = vec!();
  
  for elem in split {
    println!("{:?}", elem);
  }
  
  let mut runner = Runner::new(split_split);
}
