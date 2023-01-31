use chrono::prelude::*;
fn main() {
    let date_time: DateTime<Utc> = Utc::now(); 

    let formatted = format!("{}", date_time.format("%Y-%m-%d %H:%M:%S"));
    println!("{}", formatted);
}
