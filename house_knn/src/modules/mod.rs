pub struct Range {
  min: i32,
  max: i32,
}

pub struct Object {
  area: i32,
  rooms: i32,
  distance: i64,
  _type: String, 
}

pub struct Node {
  object: Object,
  neighbors: Vec<Object>,
}

pub struct NodeList {
  nodes: Vec<Node>,
  k: i32,
  areas: Range,
  rooms: Range,
}

impl Node {
  pub fn new() -> Node {
    Node { object: Object { area: -1, rooms: -1, distance: -1, _type: String::from("") }, neighbors: Vec::new() }
  }
  
  pub fn set_deats(&mut self, deats: &(i32, i32, String) ) {
    self.object.rooms = deats.0;
    self.object.area = deats.1;
  }
  
  pub fn sort_by_distance(&mut self) {
    self.neighbors.sort_by(|a, b| a.distance.cmp(&b.distance));
  }
  
  pub fn set_type(&mut self, new_type: String) {
    self.object._type = new_type;
  }
  
  pub fn measure_distances(&mut self, area_range_obj: &Range, rooms_range_obj: &Range) {
    let rooms_range = rooms_range_obj.max - rooms_range_obj.min;
    let area_range = area_range_obj.max - area_range_obj.min;
    for i in 0..self.neighbors.len() {
      
      let mut delta_rooms: f64 = self.neighbors[i].rooms as f64 - self.object.rooms as f64;
      delta_rooms /= rooms_range as f64;
      let mut delta_area: f64 = self.neighbors[i].area as f64 - self.object.area as f64;
      delta_area /= area_range as f64;
      
      //println!("delta area: {}, delta_rooms: {}", delta_rooms, delta_area); 

      self.neighbors[i].distance = (((delta_rooms*delta_rooms + delta_area*delta_area) as f64).sqrt() * 1000000000000000.0) as i64;
      
      //println!("Dist: {}", self.neighbors[i].distance);
    }
  }
  
  pub fn guess_type(&self, k: i32) -> String {
    let mut house = 0;
    let mut apartment = 0;
    let mut flat = 0;
    
    for i in 0..k {
      let ref neighbor = self.neighbors[i as usize];
      if neighbor._type == String::from("house") {
        house += 1;
      } else if neighbor._type == String::from("apartment"){
        apartment+=1;
      } else if neighbor._type == String::from("flat") {
        flat += 1;
      }
    }
    
    let mut guess = String::from("");
    //for i in 0..3 { loop it!!
    if house > apartment {
      if house > flat {
        guess = String::from("house");
      } else {
        guess = String::from("flat");
      }
    } else if apartment > flat {
      guess = String::from("apartment");
    } else {
      guess = String::from("flat");
    }
    
    guess
  }
}

impl NodeList {
  pub fn new(k: i32) -> NodeList {
    NodeList { nodes: Vec::new(), k: k, areas: Range { min: 1000000, max: 0 }, rooms: Range { min: 1000000, max: 0 } }
  }
  
  pub fn add(&mut self, node: Node) {
    self.nodes.push(node);
  }
  
  pub fn calc_ranges(&mut self) {
    self.areas = Range{ min: 1000000, max: 0 };
    self.rooms = Range{ min: 1000000, max: 0 };
    
    for i in 0..self.nodes.len() {
      if self.nodes[i].object.rooms < self.rooms.min {
        self.rooms.min = self.nodes[i].object.rooms;
      }
      
      if self.nodes[i].object.rooms > self.rooms.max {
        self.rooms.max = self.nodes[i].object.rooms;
      }
      
      if self.nodes[i].object.area < self.areas.min {
        self.areas.min = self.nodes[i].object.area;
      }
      
      if self.nodes[i].object.area > self.areas.max {
        self.areas.max = self.nodes[i].object.area;
      }
    }
  }

  pub fn determine_unknown(&mut self) {
    self.calc_ranges();
    for i in 0..self.nodes.len() {      
      if self.nodes[i].object._type == String::from("") {
        let mut new_neighbors: Vec<Object> = Vec::new();
        for j in 0..self.nodes.len() {
          if self.nodes[j].object._type == "" {
            continue;
          }
          {
              new_neighbors.push( Object { area: self.nodes[j].object.area, rooms: self.nodes[j].object.rooms, _type: String::from(""), distance: self.nodes[j].object.distance });
              
              if self.nodes[j].object._type == String::from("house") {
                new_neighbors[j]._type = String::from("house");
              } else if self.nodes[j].object._type == String::from("apartment") {
                new_neighbors[j]._type = String::from("apartment");
              } else if self.nodes[j].object._type == String::from("flat") {
                new_neighbors[j]._type = String::from("flat");
              }
          }
        }
        self.nodes[i].neighbors = new_neighbors;
      
        self.nodes[i].measure_distances(&self.areas, &self.rooms);
        
        self.nodes[i].sort_by_distance();
        
        println!("Area of {}m^2 with {} rooms is a {}!", self.nodes[i].object.area, self.nodes[i].object.rooms, self.nodes[i].guess_type(self.k));
      }
    }
  }
}

