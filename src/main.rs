#[macro_use] 
extern crate lazy_static;
extern crate rand;
extern crate regex;
extern crate reqwest;

mod tgk;
mod nsu;
mod errors;

fn main() {
    println!("TEMP TGK {}", tgk::current_temperature().unwrap());
    println!("TEMP NSU {}", nsu::current_temperature().unwrap());
}
