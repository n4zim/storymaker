use crate::{actor::Actor, time::Time, map::Map};

pub struct World {
  time: Time,
  #[allow(dead_code)]
  map: Map,
  #[allow(dead_code)]
  actors: Vec<Actor>,
}

impl World {
  pub fn new(map: Map) -> World {
    World {
      time: Time::new(),
      map,
      actors: Vec::new(),
    }
  }

  pub fn tick(&mut self) {
    if self.time.minute == 0 && self.time.second == 0 {
      println!("[[ DAY {} - HOUR {} ]]", self.time.day, self.time.hour);
    }
    self.time.next();
  }
}
