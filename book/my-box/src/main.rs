use std::ops::Deref;

struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}
fn hello(name: &str) {
    println!("Hello, {}!", name);
}
fn main() {
    let x = 5;
    let y = Box::new(x);
    let z = MyBox::new(x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);
    hello(&MyBox::new(String::from("Rust")));
    hello(&(*MyBox::new(String::from("Rust")))[..]);
}
