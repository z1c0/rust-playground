extern crate util;

fn main() {
	util::greeter::say("Cargo");
	println!("{}", util::text::lorem())
}