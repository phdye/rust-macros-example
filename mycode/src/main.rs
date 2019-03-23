use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[allow(dead_code)]
#[derive(HelloMacro)]
struct Pancakes {
    doable: bool,
    name: String,
    age: u32
}

fn main() {
    Pancakes::helpify();
}
