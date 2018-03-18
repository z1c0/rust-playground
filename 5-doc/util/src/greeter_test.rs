use super::greeter;

#[test]
fn greets() {
    let s = greeter::make("Test");
		assert_eq!("Hello, Test!", s)
}

#[test]
#[should_panic]
fn greets_not() {
    let s = greeter::make("test");
		assert_eq!("Hello, Test!", s)
}

#[test]
#[ignore]
fn greets_notimpl() {
		assert!(false)
}