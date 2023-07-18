use std::cell::RefCell;

thread_local!(static NOTIFY: RefCell<bool> = RefCell::new(true));

fn main() {
    println!("Hello, world!");
}
