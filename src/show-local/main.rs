use chrono::prelude::*;

fn main() {
    let date_time: DateTime<Local> = Local::now();
    
    let formatted = format!("{}", date_time.format("%Y-%m-%d %H:%M:%S"));
    println!("{}", formatted);
}
