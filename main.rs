// Write code here.
//
// To see what the code looks like after macro expansion:
//     $ cargo expand
//
// To run the code:
//     $ cargo run

// For Debugging purposes

use derive_builder::Builder;

#[derive(Builder)]
pub struct ACommand {
    executable: String,
    args: Vec<String>,
    env: Vec<String>,
    current_dir: String,
    hello: String,
}

fn main() {}
