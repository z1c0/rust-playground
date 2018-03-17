use super::greeter;

#[test]
fn greets() {
    let s = greeter::say("Test");
		assert_eq!("Hello, Test!", s)
}