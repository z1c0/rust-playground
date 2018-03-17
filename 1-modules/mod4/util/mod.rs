pub mod greeter {
	pub fn say(name : &str) {
		println!("Hello, {}!", name);
	}
}

pub mod text {
  pub fn lorem() -> &'static str {
	  "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam"
	}
}