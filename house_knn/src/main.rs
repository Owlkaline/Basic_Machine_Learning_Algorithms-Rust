extern crate rand;

mod modules;

use rand::Rng;
use modules::NodeList;
use modules::Node;

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
    let ref deats = temp_data[i];
    new_node.set_deats(deats);
    let mut vec_char: Vec<char> = Vec::new();
    for j in 0..deats.2.len() {
      vec_char.push(deats.2.chars().nth(j).unwrap());
    }    
    new_node.set_type(vec_char.iter().cloned().collect::<String>());
    nodes.add( new_node );
  }
   
  let mut rng = rand::thread_rng();
  
  for i in 0..100 {
    let random_rooms = (rng.gen::<f32>()*10.0) as i32 + 1;
    let random_area = (rng.gen::<f32>()*2000.0) as i32 + 200;
    let mut new_node: Node = Node::new();
    new_node.set_deats(&(random_rooms, random_area, String::from("")));
    new_node.set_type(String::from(""));
    nodes.add(new_node);
  }
  nodes.determine_unknown();
}

fn main() {
    println!("Hello, world!");
    
    run();
    
    
}
