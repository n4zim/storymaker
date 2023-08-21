use tokio::time::{self, Duration};

const SECONDS_PER_MINUTE: u32 = 60;
const MINUTES_PER_HOUR: u32 = 60;
const HOURS_PER_DAY: u32 = 24;

const INTERVAL: u64 = 1;

#[tokio::main]
async fn main() {
  let mut day = 1;
  let mut hour = 0;
  let mut minute = 0;
  let mut second = 0;

  let mut interval = time::interval(Duration::from_millis(INTERVAL));

  loop {
    interval.tick().await;

    if minute == 0 && second == 0 {
      println!("[[DAY {} - HOUR {}]]", day, hour);
    }

    //println!("Day {} - {:02}:{:02}:{:02}", day, hour, minute, second);

    second += 1;

    if second == SECONDS_PER_MINUTE {
      second = 0;
      minute += 1;
    }

    if minute == MINUTES_PER_HOUR {
      minute = 0;
      hour += 1;
    }

    if hour == HOURS_PER_DAY {
      hour = 0;
      day += 1;
    }
  }
}
