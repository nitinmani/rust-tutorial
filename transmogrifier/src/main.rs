// Simple command line program which takes two params
// Name of the operation and input text, then applies the operation to the text and prints result

use std::env; // how args become available - import the std::env module
use std::process::exit;
fn main() {
	let args: Vec<String> = env::args().collect(); // collect arguments into a vector of strings
	// Vector is a dynamically sized list that you can add to and remove elements from
	if args.len() != 3 {
		// first argument is the name the program was invoked with is args[0]
		eprintln!("Usage: {} <op> <text>", args[0]); // prins to standard error output
		exit(1); // exit the program. 1 means something went wrong
	}
	// types of both variables ist &String
	// String is a string type that manages its own memory allocation
	// & denotes a borrow, meaning a reference to a value. Here we're referring to the values, not pulling them out of the vector
	let op = &args[1];
	let text = &args[2];
	println!("op: {op} text: {text}");
	// borrows may not live longer than the values they are borrowing. This is one of Rust's main safety guarantees
	// immutable borrow is done with the & operator (also called shared borrows)
	// mutable (&mut) is called exclusive borrows
	/*
	let mut val = String::new();
	let ref_mut = &mut val;
	*/
	// reference does not need to be mutable so long as you do not intend to re-assign it

	// you can also shadow to lose the mutability of variables
	/*
	let mut val = String::new();
	let val = val; --> val is no longer mutable
	*/
	// match in Rust is like switch in other languages. Also happens to be an expression, like if-else if it has an else block
	// it always has to be exhaustive
	let res = match op.as_str() {
		"reverse" => text.chars().rev().collect::<String>(), // iterator over characters of the string, reversing it, collecting into another string
		"invert" => text // iterator over characters of the string, and inverting via if-ese
			.chars()
			.map(|c| {
				if c.is_uppercase() {
					c.to_lowercase().to_string()
				} else {
					c.to_uppercase().to_string()
				}
			})
			.collect::<String>(),
		"uppercase" => text.to_uppercase(),
		"no-spaces" => text
			.chars()
			.filter(|c| !c.is_whitespace()) // filter also takes a closure which returns a boolean. Keep the true values
			.collect::<String>(),
		"leet" => text
			.chars()
			.map(|c| match c {
				'a' | 'A' => '4',
				'e' | 'E' => '3',
				'i' | 'I' => '1',
				'o' | 'O' => '0',
				's' | 'S' => '5',
				't' | 'T' => '7',
				_ => c
			})
			.collect::<String>(),
		"acronym" => text
			.split_whitespace() // splits text into words by splitting on whitespace 
			.map(|word| word.chars().next().unwrap()) // first character of each word. unwrap() needed because next() returns Option<char>. Option is a type that indicates a value may not be present since iterator might be empty. unwrap() tells Rust to safely shutdown program if empty
			.collect::<String>()
			.to_uppercase(),
		_ => {
			eprintln!("Invalid opeation: {}", op);
			exit(1);
		}
	};

	println!("{}", res);
}

/*
Errors in Rust have a code and we can ask the compiler to explain the error to use
rustc --explain E0381

It is not allowed to use or capture an uninitialized variable.

Erroneous code example:

```
fn main() {
    let x: i32;
    let y = x; // error, use of possibly-uninitialized variable
}
```

To fix this, ensure that any declared variables are initialized before being
used. Example:

```
fn main() {
    let x: i32 = 0;
    let y = x; // ok!
}


*/