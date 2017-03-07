extern crate rand;

mod modules;

use rand::Rng;
use modules::Population;

fn main() {    
    let mut population = Population::new(String::from("Hello, world!"), 20);
    
    let mut finished: bool = false;
    
    while !finished {
      finished = population.generation();
    }
}
