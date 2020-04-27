#[macro_use]
extern crate clap;

mod arg;

use arg::parse_arguments;

fn main() {
	let args = parse_arguments();
	println!("Running with arguments: {:?}", args);
}
