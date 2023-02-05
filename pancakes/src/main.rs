use hello_macro_derive::HelloMacro;
use hello_macro::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;
fn main() {
    Pancakes::hello_macro()
}
