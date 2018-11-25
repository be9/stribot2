#[macro_use] 
extern crate lazy_static;
extern crate regex;
extern crate reqwest;

mod tgk;

fn main() {
    println!("TEMP {}", tgk::current_temperature().unwrap());
}
