use chrono::{Utc,Local};
use dotenv::dotenv;
use std::env;

fn main(){
    let utc =Utc::now();
    let local=Local::now();
    println!("{}",utc);
    println!("{}",local);
    dotenv().ok();
    let data = env::var("GOOGLE_SECRET");
    match data{
      Ok(a)=>println!("{}",a),
      Err(_e)=>println!("{}","something error ")
    }
}