use std::io::{self, Read};

fn main() {
	let mut input = String::new();
	println!("Please enter some text:");
	match io::stdin().read_line(&mut input) {
		Ok(num_of_characters) => {
			println!("You typed: {}", input.trim());
			println!("num_of_characters: {}", num_of_characters);
		},
		Err(error) => println!("Error reading from stdin: {}", error),
	}
}