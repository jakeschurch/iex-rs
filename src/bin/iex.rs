extern crate iex;
use iex::*;

fn main() {
    let iex = IexClient::new().unwrap();
    println!("{:?}", iex.earnings("aapl").unwrap());
}
