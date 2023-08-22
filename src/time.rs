
const SECONDS_PER_MINUTE: u32 = 60;
const MINUTES_PER_HOUR: u32 = 60;
const HOURS_PER_DAY: u32 = 24;

pub struct Time {
  pub day: i32,
  pub hour: u32,
  pub minute: u32,
  pub second: u32,
}

impl Time {
  pub fn new() -> Time {
    Time {
      day: 1,
      hour: 0,
      minute: 0,
      second: 0,
    }
  }

  pub fn next(&mut self) {
    self.second += 1;

    if self.second == SECONDS_PER_MINUTE {
      self.second = 0;
      self.minute += 1;
    }

    if self.minute == MINUTES_PER_HOUR {
      self.minute = 0;
      self.hour += 1;
    }

    if self.hour == HOURS_PER_DAY {
      self.hour = 0;
      self.day += 1;
    }
  }

  #[allow(dead_code)]
  pub fn print(&self) {
    println!(
      "Day {} - {:02}:{:02}:{:02}",
      self.day,
      self.hour,
      self.minute,
      self.second,
    );
  }
}
