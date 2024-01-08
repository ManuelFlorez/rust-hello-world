use time::OffsetDateTime;
use chrono::{Local, DateTime};

fn main() {
    let now = OffsetDateTime::now_utc();
    let local: DateTime<Local> = Local::now();
    
    println!("{now}");
    println!("{local}");
}
