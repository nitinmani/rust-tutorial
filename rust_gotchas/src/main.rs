fn print_my_string(string: &String) {
	// use &str for flexibility as &String can convert to it automatically 
	println!("{}", string); // compare to const char * const string, which would be C equivalent
	// string reference destroyed here
}

fn main() {
	// Rust does not have garbage collector but has different model to the model used in C
	// Memory management is lexical. Memory is allocated when a variable (or value) is created and freed when it goes out of scope in terms of syntax unless moved
	// Borrow Checker: when something is out of scope by the Rust compiler, it ensures that there are no dangling pointers 

	// Rust enforces RAII (Resource Acquisition is Initialization) --> initializing a variable gives you memory or other resources and when object goes out of scope, destructor is called and resources returned to system

	let the_11th_commandement = String::from("Braiins OS rocks!"); // allocated memory, I own the string in this function
	// memory used by the string will be freed here, since ownership is not passed elsewhere and main() ends here

	// references to a resource depend on the lifetime of that resource (only valid until resource is deallocated)

	print_my_string(&the_11th_commandement);
	print_my_string(&the_11th_commandement);

	let string_ref = &the_11th_commandement;
	print_my_string(string_ref);
	// string_ref is destroyed here
	// the_11th_commandment is destroyed here

	/*
	This function won't compile. We have to specify a lifetime explicitly here through the 'name syntax in the <> brackets
	Otherwise, Rust assumes you maybe want to return constants, which have a 'static lifetime and as such live forever
	fn give_me_a_ref<'a>() -> &'a String {
		let temp =
			String::from("Opps want an initiative - blow up their entire quadrant!");

		&temp // same as return &temp;
		// <- temp would be freed here,
		// the returned reference cannot outlive it
	}

	This is different to C, which uses copy semantics by default (to give a parameter to a function means to create a copy which is available in the function)
	
	In Rust you take the value you have and give it to a function, and then you can no longer access it
	main() owns x until completely_safe_storage() is called, at which point ownership is haded to it (x is moved into the function) and completely_safe_storage() owns x until dropped

	fn completely_safe_storage(value: String) {
		// <- value is immediately freed
	}
	fn main() {
		let x = String::from("1337 US Dollars");
		completely_safe_storage(x);
		// ↑ ownership of x was moved to completely_safe_storage()
		println!("{}", x);
		// ↑ this does not compile, as we no longer have the ownership of x
	}
	
	To not move a resource, you instead have to use borrowing. Create a reefrence to it and move that. You own the reference when you create it, and then you move it and ownership of it to the function you call
	*/

	// To manipulate a resource without giving up ownership, you can create one mutable reference. During the lifetime of this reference, no other references to the same resource can exist

	let mut bitcoin = String::from("bitcoin");

	// if Rust sees you're not using mut_ref after you have created ro_ref, it destroys it early
	let mut_ref = &mut bitcoin; // borrow bitcoin mutably
	let ro_ref = &bitcoin; // borrow bitcoin immutably like char* const ptr in C
	// this example will not compile since bitcoin already borrowed mutably
	mut_ref.push_str(", the cryptocurrency");

	// Lifetime denotes how long a resource exists or is accessible from start to finish. Spoken about in terms of references
	// 'ident in Rust denotes lifetimes. They usually appear as generic parameters
	// You can only verify that lifetimes are equal or if one satisfies the other ('a lives longer than 'b)
	// 'static denotes references that are valid for the entirety of the program's run from anywhere. Generally through constants and statics
	// static NUMBER_REF: &'static i32 = &42;

	/*
	fn main() {
		'bitcoin_lifetime: {
			let mut bitcoin = String::from("bitcoin");
			'mut_ref_lifetime: {
				let mut_ref = &mut bitcoin;
				// ↑ borrow bitcoin mutably
				'ro_ref_lifetime: {
					let ro_ref = &bitcoin;
					// ↑ borrow bitcoin immutably
					println!("{}", ro_ref); // <- use the immutable borrow
					mut_ref.push_str(", the cryptocurrency");
					// ↑ use the mutable borrow
				} // <- ro_ref goes out of scope here
			} // <- mut_ref goes out of scope here. mut_ref and ro_ref cannot co-exist
		} // <- bitcoin goes out of scope here
	}
	*/
}

// for two references, left and right, which live the same, return a reference that lives as long as these two
fn max_ref<'a>(left: &'a i32, right: &'a i32) -> &'a i32 {
	if *left < *right {
		right
	} else {
		left
	}
}

// for two lifetimes 'a and 'b such that 'a lives as long as 'b or longer
fn foobar<'a, 'b>(_x: &'a i32, _y: &'b i32)
where
	'a: 'b
{ 
	// code
}