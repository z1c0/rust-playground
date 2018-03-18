//! Dieses Modul liefert einen "Hello, World!" Begrüßungstext.
//!
//! # Beispiel
//!
//! ```
//! use util::greeter;
//! assert_eq!("Hello, Test!", greeter::make("Test"));
//! ```


/// Gibt den Begrüßungstext direkt auf der Konsole aus.
pub fn say(name : &str) {
	println!("{}", make(name))
}

/// Formatiert den Begrüßungstext.
///
/// ```
/// use util::greeter;
/// assert_eq!("Hello, Test!", greeter::make("Test"));
/// ```
pub fn make(name : &str) -> String {
	format!("Hello, {}!", name)
}

