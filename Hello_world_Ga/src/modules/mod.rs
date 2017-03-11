extern crate rand;

use rand::Rng;
use std::{thread, time};

pub struct Chromosome {
  code: String,
  cost: i32,
}

pub struct Population {
  members: Vec<Chromosome>,
  goal: String,
  generation_number: i32,
  last_gen_children: [Chromosome; 2],
}

impl Population {
  pub fn new(goal: String, size: i32) -> Population {
    let mut new_members: Vec<Chromosome> = Vec::new();
    for i in 0..size {
      let mut chromosome = Chromosome::new();
      chromosome.randomise(goal.len());
      new_members.push(chromosome);
    }
    Population { members: new_members, goal: goal, generation_number: 0, last_gen_children: [Chromosome::new(), Chromosome::new()] }
  }
  
  pub fn sort(&mut self) {
    self.members.sort_by(|a, b| a.cost.cmp(&b.cost));
  }
  
  pub fn average(&self) -> i32 {
    let mut average_cost = 0;
    for i in 0..self.members.len() {
      average_cost += self.members[i].cost;
    }
    average_cost / self.members.len() as i32
  }
  
  pub fn splice(&mut self, start_point: usize) {
    for i in start_point..self.members.len() {
      let mut final_code: Vec<char> = Vec::new();
      {
        let mems = &mut *self.members;
        for j in 0..mems[0].code.len() {
          if i % 2 == 0 {
            final_code.push(self.last_gen_children[0].code.chars().nth(j).unwrap());
          } else {
            final_code.push(self.last_gen_children[1].code.chars().nth(j).unwrap());
          }
        }     
      }
      self.members[i].change_code(final_code); 
    }
  }
  
  pub fn generation(&mut self) -> bool {
    for i in 0..self.members.len() {
      self.members[i].calc_cost(&self.goal);
    }
    
    self.sort();
    
    if self.generation_number % 100 == 0 {
      self.display();
    }
    
    self.last_gen_children = self.members[0].mate(&self.members[1]);
    self.splice(2);
    
    
    for i in 0..self.members.len() {
      self.members[i].mutate(0.5);
      self.members[i].calc_cost(&self.goal);
      if self.members[i].code == self.goal {
        //sort
        self.sort();
        self.average();
        //display
        self.display();
        println!("Goal String reached in {} generations", self.generation_number);
        return true
      }
    }
    self.generation_number+=1;
    let some_millis = time::Duration::from_millis(10);
    thread::sleep(some_millis);

    false
  }
  
  pub fn display(&self) {
    
    println!("Generation: {}\n    Avg Cost: {} \n    Best String: {}\n    Goal String: {}\n", self.generation_number, self.average(), self.members[0].code, self.goal);
  }
}

impl Chromosome {
  pub fn new() -> Chromosome {
    Chromosome { code: String::from(""), cost: 9999 }
  }
  
  pub fn randomise(&mut self, length: usize) {
    let mut rng = rand::thread_rng();
    self.code = String::from("");
    for i in 0..length {
      self.code.push(rng.gen_range(b' ', b'z') as char);
    }
  }
  
  pub fn calc_cost(&mut self, compare_to: &str) {
    let mut total: i32 = 0;
    for i in 0..self.code.len() {
      total += (self.code.chars().nth(i).unwrap() as i32 - compare_to.chars().nth(i).unwrap() as i32) * 
               (self.code.chars().nth(i).unwrap() as i32 - compare_to.chars().nth(i).unwrap() as i32);
    }
    self.cost = total;
  }
  
  pub fn change_code(&mut self, new_code: Vec<char>) {
    self.code = new_code.iter().cloned().collect::<String>();
  }
  
  pub fn mate(&self, partner: &Chromosome) -> [Chromosome; 2] {
    let pivot = (self.code.len() / 2) as i32 - 1;
    
    let mut child1 = String::from("");
    let mut child2 = String::from("");
    
    for i in 0..self.code.len() {
      if i <= pivot as usize {
        child1.push(self.code.chars().nth(i).unwrap());
        child2.push(partner.code.chars().nth(i).unwrap());
      } else {
        child1.push(partner.code.chars().nth(i).unwrap());
        child2.push(self.code.chars().nth(i).unwrap());
      }
    }
    
    let mut chromo1 = Chromosome::new();
    let mut chromo2 = Chromosome::new();
    chromo1.code = child1;
    chromo2.code = child2;
    
    [chromo1, chromo2]
  }
  
  pub fn mutate(&mut self, chance: f32) {
    let mut rng = rand::thread_rng();
    if rng.gen::<f32>() > chance {
      return;
    }
    
    let index = (rng.gen::<f32>()*self.code.len() as f32) as usize;
    //let upOrDown = rng.gen::<f32>();
    let mut rng = rand::thread_rng();
    let mut new_string: String = String::from("");
    
    for i in 0.. self.code.len() {
      if i != index {
        new_string.push(self.code.chars().nth(i).unwrap());
      } else {
        new_string.push(rng.gen_range(b' ', b'z') as char);
      }
    }
    self.code = new_string;
  }
}

