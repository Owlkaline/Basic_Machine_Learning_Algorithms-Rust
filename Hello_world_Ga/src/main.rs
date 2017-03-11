extern crate rand;

mod modules;

use modules::Population;
use std::time::Instant;
use std::io;

fn main() {
    println!("Simple genetic algoithm");
    println!("Obtains the goal string from a randomly generated string.");
    println!("Example Goal stirng: Hello, world!");
    
    println!("\nInput Goal string: ");
    
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .ok()
        .expect("Couldn't read line");
    
    let mut population = Population::new(String::from(input.trim()), 20);
    
    let mut finished: bool = false;
    
    let start = Instant::now();
    while !finished {
      finished = population.generation();
    }
    let elapsed = start.elapsed();
    
    println!("Elapsed time: {} ms", (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64);
} 
