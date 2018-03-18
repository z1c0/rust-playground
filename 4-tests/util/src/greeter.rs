pub fn say(name : &str) {
	println!("{}", make(name))
}

pub fn make(name : &str) -> String {
	format!("Hello, {}!", name)
}
