extern crate iex;
use iex::*;

fn main() {
    let iex = IexClient::new().unwrap();
    println!("{:?}", iex.stats("woot").unwrap());
}
