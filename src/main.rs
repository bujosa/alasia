use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct DragonBall;

fn main() {
    DragonBall::hello_macro();
}
