use crate::token::Token;
use cli_toolbox::system::{self, System};

mod token;

fn main() {
    let mut sys: System = system::System::new("sys".to_string(), None, None);
    sys.add_program("eyyy".to_string(), first_prog, None);
    sys.menu();
    println!("Hello, world!");
    println!();
}

fn first_prog() {
    println!("hey !");
}
