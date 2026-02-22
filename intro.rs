fn factorial_iter(num: usize) -> usize {
	(1..num)
		.fold(1, |acc, x| acc * x)
}

fn factorial_loop(num: usize) -> usize {
	let mut prod = 1;
	for x in 2..num {
		prod *= x;
	}
	prod
}

fn fibonacci(n: usize) -> usize {
	let mut a = 1;
	let mut b = 1;

	for _ in 1..n {
		let old_a = a;
		a = b;
		b += old_a;
	}
	b
}

fn main() {
	let x = factorial_iter(12);
	let y = factorial_loop(20);
	let fib = fibonacci(35);

	/**
	What's interesting is that the assembly code looks like this

	mov qword ptr [rsp], 39916800
	movabs rax, 121645100408832000
	mov qword ptr [rsp + 8], rax
	mov qword ptr [rsp + 16], 14930352

	Rust is able to evaluate the algorithms at compile time and embed the results directly into the binary

	Effectively, it's like our main() method is just this:
	print("factorial 1: 39916800")
	print("factorial 2: 121645100408832000")
	print("fibbonaci: 14930352")
	print("\n")
	**/
	println!("factorial 1: {}, factorial 2: {}, fibonacci: {}", x, y, fib);
}

/**
The toolchain comes bundled with the Cargo tool. Cargo is a package manager and build system. Fetches and configures dependencies
cargo install <name> installs the package on the system

crate is the smallest unit of compilation and packaging. It's a package of Rust code that the compiler treats as a single unit
crates can be compiled into binary executables or into libraries that other crates can link against

1. binary crates
	applications that you can compile into a standalone executable
	has a main.rs file as the entry point with the main() function where execution begins

2. library crates
	collections of code intended to be used as dependencies by other crates
	no main() function and instead provides functions, types, and constants that can be used by other crates
	entry point is typically lib.rs

crates can have both binary and library part
there can be multiple executable binaries produced by create, but only one library

Crates can be published on crates.io (Rust's official package registry)
Crates have support for modularity and resuability. They also can support versioning and dependency management
**/

/**
Dependency management in Rust is pretty streamlined and efficient because of Cargo
Dependencies for a Rust project are in Cargo.toml
	contains all the necessary information about a project including which external crates the project depends on

Cargo.toml
	[package]
	name = "my_project"
	version = "0.1.0"
	edition = "2021"
	[dependencies]
	serde = "1.0" # a library for de/serialization
	log = "0.4" # a logging facade, in binary crates, you need to add
	an implementation crate, too

project depends on two crates, serde and log, with specified versions
explicitness in declaring dependencies makes it clear what the project needs to build and run, avoiding ambiguity and ensuring consistency

cargo add serde_json --> automatically finds the latest version of serde_json to add to Cargo.toml. Do this over manually editing the file
cargo update --> update project's dependencies to their latest permissible version
cargo build --> compile your project along with its dependencies
cargo run --> run the project directly
**/

/**
cargo new <project_name> Creates a new Rust project in a new directory.
cargo init Initializes a new Rust project in the current directory.
cargo build Compiles the current project and all of its dependencies.
cargo run Compiles and runs the current project.
cargo test Runs the tests for the current project.
cargo check Quickly checks your code to ensure it compiles butdoes not produce an executable.
cargo clean Removes the target directory with the compiled artifacts.
cargo update Updates dependencies as recorded in the local lock file.
cargo doc Generates documentation for the current project‘s dependencies.
cargo publish Packages and uploads the current project to crates.io.
cargo bench Runs the benchmarks of the current project. (Note: Requires a nightly build of Rust.)
**/

// if you create Cargo.toml and it lists crates that don't exist yet, Cargo will complain until all of the member crates exist