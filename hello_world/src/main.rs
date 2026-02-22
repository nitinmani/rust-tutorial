fn main() {
	let my_name = "Nitin"; // let my_name: &str = "Nitin";
    println!("Hello, {}!", my_name);

    // println!("Hello, {my_name}!"); also works

    /*
	let my_name = "Nitin";
	println!("Hello, {}!")

	error: 1 positional argument in format string, but no arguments
	were given
	--> hello_world/src/main.rs:4:22
	|
	4 | println!("Hello, {}!");
	| ^^
	error: could not compile `hello_world` (bin "hello_world") due to
	1 previous error

	compiler says it expected something to be substituted inside the format string but nothing was provided

	in C if you forget to provide an argument to printf() it will still compile. Rust doesn't!
	*/

	/*
	let my_name = "Nitin";
	println!("Hello, world!", my_name)

	error: argument never used
	--> hello_world/src/main.rs:4:31
	|
	4 | println!("Hello, world!", my_name);
	| --------------- ^^^^^^^ argument never used
	| |
	| formatting specifier missing
	error: could not compile `hello_world` (bin "hello_world") due to
	1 previous error
	*/
}

// fn declares a function
// function is named main and takes no arguments
// function argument lists are followed by the code block delimited by {}
// println! is not a function call but is rather a macro
// macro because Rust has no variadic functions (function that can take any number of parameters and process one by one) AND macro does useful compile-time checks

/*
Variables

let my_name: &str = "Nitin" --> string slice
let age: i32 = 30; --> 32-bit signed integer
let temperature: f64= 20.5; --> 64 bit floating point
let is_active: bool = true; --> boolean type
let initial: char = 'A'; --> char represents Unicode scalar value
let count: u32 = 100; --> 32-bit unsigned integer
let distance = 15.0; --> Rust automatically infers distance to be f64
*/

/*
fn main() {
	let my_name: &str = "Nitin";
	// As all know, carcinization is the final stage of evolution,
	// and I am already growing pincers B)
	my_name = "Ferris";
	println!("Hello, {my_name}!");
}

This does not compile. We get a warning that the original value of the variable is never read (not an error but a waste)
Then we get an error saying we cannot assign twice to an immutable variable because in Rust, variables and arguments are immutable by default

One solution is just to make it immutable

The second is to just declare it as mutable

fn main() {
	let mut my_name = "Walter White";
	println!("My name is {my_name}. I live at \
		308 Negra Arroyo Lane, \
		Albuquerque, \
		New Mexico. \
		87104."
	)
	my_name = "Ferris";
	println!("Hello, {my_name}!");
}

This prints out:

My name is Walter White. I live at
	308 Negra Arroyo ane,
	Albuquerque,
	New Mexico,
	87104.
Hello Ferris!

Without the \ it'll be this:
My name is Walter Hartwell White. I live at 308 Negra Arroyo Lane, Albuquerque, New Mexico, 87104
Hello, Ferris!
*/
