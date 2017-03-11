extern crate rand;

mod modules;

use modules::Population;
use std::time::Instant;

fn main() {    
    let mut population = Population::new(String::from("Hello, world!"), 20);
    
    let mut finished: bool = false;
    
    let start = Instant::now();
    while !finished {
      finished = population.generation();
    }
    let elapsed = start.elapsed();
    
    println!("Elapsed time: {} ms", (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64);
} 
