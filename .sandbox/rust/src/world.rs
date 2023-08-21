use crate::{actor::Actor, time::Time, map::Map};

pub struct World {
  time: Time,
  actors: Vec<Actor>,
}

impl World {
  pub fn new(map: Map) -> World {
    World {
      actors: Vec::new(),
      time: Time::new(),
    }
  }

  pub fn tick(&mut self) {
    if self.time.minute == 0 && self.time.second == 0 {
      println!("[[ DAY {} - HOUR {} ]]", self.time.day, self.time.hour);
    }
    self.time.next();
  }
}
