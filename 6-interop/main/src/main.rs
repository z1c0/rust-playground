use std::ffi::CString;

#[link(name = "user32")]
extern {
   fn MessageBoxA(hwnd: i32, text : *const i8, caption : *const i8, _type : i32) -> i32;
}

fn message_box(msg: &str) {	
		let cstr = CString::new(msg).unwrap();
		unsafe {			
    	MessageBoxA(0, cstr.as_ptr(), cstr.as_ptr(), 0);
		}
}

fn main() {
    message_box("Hello from Rust!");
}