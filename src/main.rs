// use std::env; // to get arugments passed to the program
// use std::thread;
// use std::sync::Mutex;
// use std::sync::Arc;
// use std::thread::JoinHandle;

extern crate reqwest;

fn main() {
    let wrs = reqwest::blocking::get("https://www.fantasypros.com/nfl/projections/wr.php").unwrap().text().unwrap();
    println!("{}", wrs);
}
