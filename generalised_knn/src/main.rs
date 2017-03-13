extern crate rand;

mod modules;

use rand::Rng;
use modules::NodeList;
use modules::Node;

fn return_string(old_str: &String) -> String {
  let mut vec_char: Vec<char> = Vec::new();
  for i in 0..old_str.len() {
    vec_char.push(old_str.chars().nth(i).unwrap());
  }
  vec_char.iter().cloned().collect::<String>()
}

fn run() {
  let mut data: Vec<(i32, i32, String)> = Vec::new();
    data.push(( 1,  350,  String::from("apartment"))); 
    data.push(( 2,  300,  String::from("apartment")));
    data.push(( 3,  300,  String::from("apartment")));
    data.push(( 4,  250,  String::from("apartment")));
    data.push(( 4,  500,  String::from("apartment")));
    data.push(( 4,  400,  String::from("apartment")));
    data.push(( 5,  450,  String::from("apartment")));

    data.push(( 7,   850,   String::from("house")));
    data.push(( 7,   900,   String::from("house")));
    data.push(( 7,   1200,  String::from("house")));
    data.push(( 8,   1500,  String::from("house")));
    data.push(( 9,   1300,  String::from("house")));
    data.push(( 8,   1240,  String::from("house")));
    data.push(( 10,  1700,  String::from("house")));
    data.push(( 9,   1000,  String::from("house")));

    data.push(( 1,  800,   String::from("flat")));
    data.push(( 3,  900,   String::from("flat")));
    data.push(( 2,  700,   String::from("flat")));
    data.push(( 1,  900,   String::from("flat")));
    data.push(( 2,  1150,  String::from("flat")));
    data.push(( 1,  1000,  String::from("flat")));
    data.push(( 2,  1200,  String::from("flat")));
    data.push(( 1,  1300,  String::from("flat")));
  
  let mut nodes: NodeList = NodeList::new(3);
  let temp_data = &mut *data;
  for i in 0..temp_data.len() {
    let mut new_node: Node = Node::new();
    let mut deats = Vec::new();
    deats.push(temp_data[i].0);
    deats.push(temp_data[i].1);
    
    new_node.set_deats(deats); 
    new_node.set_result(return_string(&temp_data[i].2));
    
    nodes.add( new_node );
  }
   
  let mut rng = rand::thread_rng();
  
  for i in 0..1 {
    let mut unknown_deats: Vec<i32> = Vec::new();
    unknown_deats.push((rng.gen::<f32>()*10.0) as i32 + 1);
    unknown_deats.push((rng.gen::<f32>()*2000.0) as i32 + 200);
    
    let mut new_node: Node = Node::new();
    new_node.set_deats(unknown_deats);
    new_node.set_result(String::from(""));
    nodes.add(new_node);
  }
  nodes.determine_unknown();
}

fn main() {
    println!("Hello, world!");
    
    run();
    
    
}
