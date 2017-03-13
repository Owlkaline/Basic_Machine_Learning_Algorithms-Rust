pub struct Range {
  min: i32,
  max: i32,
}

pub struct Object {
  attributes: Vec<i32>,
  distance: i64,
  result: String, 
}

pub struct Node {
  object: Object,
  neighbors: Vec<Object>,
}

pub struct NodeList {
  nodes: Vec<Node>,
  k: i32,
  ranges: Vec<Range>,
  possible_results: Vec<String>,
}

impl Node {
  pub fn new() -> Node {
    Node { object: Object { attributes: Vec::new(), distance: -1, result: String::from("") }, neighbors: Vec::new() }
  }
  
  pub fn set_deats(&mut self, deats: Vec<i32> ) {
    for i in 0..deats.len() {
      self.object.attributes.push(deats[i] as i32);
    }
  }
  
  pub fn return_string(&self, old_str: &String) -> String {
    let mut vec_char: Vec<char> = Vec::new();
    for i in 0..old_str.len() {
      vec_char.push(old_str.chars().nth(i).unwrap());
    }
    vec_char.iter().cloned().collect::<String>()
  }
  
  pub fn sort_by_distance(&mut self) {
    self.neighbors.sort_by(|a, b| a.distance.cmp(&b.distance));
  }
  
  pub fn set_result(&mut self, new_result: String) {
    self.object.result = new_result;
  }
  
  pub fn measure_distances(&mut self, ranges: &Vec<Range>) {
    let mut attri_ranges: Vec<i32> = Vec::new();
    
    for i in 0..ranges.len() {
      attri_ranges.push(ranges[i].max - ranges[i].min);
    }
    
    let mut total: f64 = 0.0;
    for i in 0..self.neighbors.len() {
      for j in 0..self.neighbors[i].attributes.len() {
        total += ((self.neighbors[i].attributes[j] as f64 - self.object.attributes[j] as f64) / attri_ranges[j] as f64) *
                 ((self.neighbors[i].attributes[j] as f64 - self.object.attributes[j] as f64) / attri_ranges[j] as f64);
      }
    
      self.neighbors[i].distance = (total.sqrt() * 1000000000000000.0) as i64;
    }
  }
  
  pub fn guess_type(&self, k: i32, p_results: &Vec<String>) -> String {
    let mut strongest_atri: Vec<i32> = Vec::new();
    
    for i in 0..p_results.len() {
      strongest_atri.push(0);      
    }
    
    for i in 0..k {
      let ref neighbor = self.neighbors[i as usize];
      
      for j in 0..p_results.len() {
        println!("{}", neighbor.result);
        if neighbor.result == p_results[j] {
          strongest_atri[j]+=1;
          break;
        }
      }
    }
    
    let mut guess = String::from("");
    let mut best = 0;

    for i in 0..strongest_atri.len() {
      if strongest_atri[i] > best {
        best = strongest_atri[i];
        guess = self.return_string(&p_results[i]);
      }
    }
    
    guess
  }
}

impl NodeList {
  pub fn new(k: i32) -> NodeList {
    NodeList { nodes: Vec::new(), k: k, ranges: Vec::new(), possible_results: Vec::new() }
  }
  
  pub fn add(&mut self, node: Node) {
    self.nodes.push(node);
  }
  
  pub fn calc_ranges(&mut self) {
    for i in 0..self.nodes[0].object.attributes.len() {
      self.ranges.push(Range{ min: 1000000, max: 0 });
    }
    
    for i in 0..self.nodes.len() {
      for j in 0..self.ranges.len() {
        if self.nodes[i].object.attributes[j] < self.ranges[j].min {
          self.ranges[j].min = self.nodes[i].object.attributes[j];
        }
        
        if self.nodes[i].object.attributes[j] < self.ranges[j].max {
          self.ranges[j].max = self.nodes[i].object.attributes[j];
        }
      }
    }
    
    for i in 0..self.nodes.len() {
      if self.nodes[i].object.result == String::from("") {
        continue;
      }
      for j in 0..self.possible_results.len() {
        if self.nodes[i].object.result == self.possible_results[j] {
          continue;
        }
      }
      self.possible_results.push(self.nodes[i].return_string(&self.nodes[i].object.result));
    }
  }

  pub fn determine_unknown(&mut self) {
    self.calc_ranges();
    
    for i in 0..self.nodes.len() {      
      if self.nodes[i].object.result == String::from("") {
        let mut new_neighbors: Vec<Object> = Vec::new();
        let mut neighbor_num = 0;
        for j in 0..self.nodes.len() {
          if self.nodes[j].object.result == "" {
            continue;
          }
          {
              let temp_data = &mut *self.nodes;
              
              new_neighbors.push( Object { attributes: Vec::new(), result: String::from(""), distance: temp_data[j].object.distance });
              for k in 0..temp_data[j].object.attributes.len() {
                new_neighbors[neighbor_num].attributes.push(temp_data[j].object.attributes[k]);
                new_neighbors[neighbor_num].result = temp_data[j].return_string(&temp_data[j].object.result);
              }
          }
          neighbor_num+=1;
        }
        self.nodes[i].neighbors = new_neighbors;
      
        self.nodes[i].measure_distances(&self.ranges);
        
        self.nodes[i].sort_by_distance();
        
        println!("room is a {}!", self.nodes[i].guess_type(self.k, &self.possible_results));
      }
    }
  }
}

