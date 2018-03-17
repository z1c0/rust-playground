extern crate util;

fn main() {
	util::greeter::say("Crate");
	println!("{}", util::text::lorem())
}