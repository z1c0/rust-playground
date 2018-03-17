mod util;

fn main() {
	util::greeter::say("Rust");
	println!("{}", util::text::lorem())
}