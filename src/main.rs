extern crate chrono;
extern crate clap;
#[macro_use] extern crate lazy_static;
extern crate rand;
extern crate regex;
extern crate reqwest;
extern crate select;

use errors::StribotError;
use std::sync::mpsc;
use std::thread;

use clap::{App, SubCommand};

mod tgk;
mod nsu;
mod errors;

fn main() {
    let matches = App::new("Stribot")
                          .version("2.0")
                          .author("Oleg Dashevskii <be9@be9.ru>")
                          .about("A weather telegram bot")
                          .subcommand(SubCommand::with_name("now")
                                      .about("Shows current temperature"))
                          .subcommand(SubCommand::with_name("minmax")
                                      .about("Shows min and max temperature"))
                          .get_matches();

    if let Some(_) = matches.subcommand_matches("now") {
        temperature();
    }

    if let Some(_) = matches.subcommand_matches("minmax") {
        minmax();
    }
}

#[derive(Debug)]
struct TempResult {
    name: &'static str,
    value: Result<f64, StribotError>,
}

fn temperature() {
    let (tx1, rx) = mpsc::channel();
    let tx2 = mpsc::Sender::clone(&tx1);

    let handle1 = thread::spawn(move || {
        tx1.send(TempResult {
            name: "ТГК",
            value: tgk::current_temperature(),
        }).unwrap();
    });

    let handle2 = thread::spawn(move || {
        tx2.send(TempResult {
            name: "НГУ",
            value: nsu::current_temperature(),
        }).unwrap();
    });

    let t1 = rx.recv();
    println!("{:?}", t1);

    let t2 = rx.recv();
    println!("{:?}", t2);

    handle1.join().unwrap();
    handle2.join().unwrap();
}

fn minmax() {
    println!("MINMAX ТГК {:?}", tgk::current_minmax().unwrap());
}
