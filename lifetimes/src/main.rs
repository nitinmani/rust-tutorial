fn example1() {
	let r: &i32;														// ---------+--- 'a
																		//			|
	{																	//			|
		let x: i32 = 5;													// -+-- 'b	|
		r = &x;															//	|		|
	}																	// -+		|
	// x is invalidated here. Does not live long enough to print Rust   //			|
	// Rust knows this as compile time									//			|
																		//			|
	println!("r: {}", r);												//----------+
}

fn example2() {
	let x: i32 = 5;			//---------+-- 'b
	let r: &i32 = &x;		//--+-- 'a |
	println!("r: {}", r);	//	|	   |
							//---------+
}


fn main() {
	let string1: String = String::from("abcd");
	let string2: String = String::from("xyz");

	let result: &str = longest2(x: string1.as_str(), y: string2.as_str()); // lifetime of result?
	// lifetime of result is lifetime of the smallest lifetime passed in, string1 and string2
	// here they're the same, but they could be different
	println!("The longest string is {}", result);
}

fn longest_wrong(x: &str, y: &str) -> &str {	// x can have a different lifetime than y. Which are we returning?
	if x.len() > y.len() {						// missing lifetime specifier. Function's return type contains borrowed value but signature doesn't say if borrowed from x or y
		x
	} else {
		y
	}
}

// Need to use generic lifetime annotation. Describe the relationship between multiple lifetimes
// &i32 is a reference
// &'a i32 is a reference with explicit lifetime
// &'a mut i32 is a mutable reference with explicit lifetime 

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
	// there's a relationship between x, y, and the return value
	// the lifetime of the return reference is the same as the smallest lifetime of the arguments
	if x.len() > y.len() {
		x
	} else {
		y
	}
}

fn main() {
	let string1: String = String::from("abcd");
	{
		let string2: String = String::from("xyz");
		let result: &str = longest(x: string1.as_str(), y: string2.as_str());
		// result has lifetime of string2 because it is smaller
		// print will execute
		println!("The longest string is {}", result);
	}
}

fn main() {
	let string1: String = String::from("abcd");
	let result: &str;
	{
		let string2: String = String::from("xyz");
		result = longest(x: string1.as_str(), y: string2.as_str());
		// result has lifetime of string2 because it is smaller
	}
	// this will not compile because string2 does not live long enough for this to be valid
	println!("The longest string is {}", result);
}

fn main() {
	let string1: String = String::from("abcd");
	let result: &str;
	{
		let string2: String = String::from("xyz");
		result = returnX(x: string1.as_str(), y: string2.as_str());
		// result has lifetime of string2 because it is smaller
	}
	// code is now valid
	// returnX is going to a reference that has the same lifetime as x which is string1
	// string1 is valid here
	println!("The longest string is {}", result);
}

fn returnX<'a>(x: &'a str, : &str) -> &'a str {
	x
}

// Lifetime of the return value always has to be tied back to lifetime of parameters
// If we pass back reference from a function it is in reference to something passed in
// You cannot return a reference to something created in the function

fn returnResult<'a>(x: &str, y: &str) -> &'a str {
	let result: String = String::from("really long string");
	result.as_str(); // cannot value referencing local variable result. Once function is over the local variables get destroyed 
}

fn returnResultAsString<'a>(x: &str, y: &str) -> String {
	let result: String = String::from("really long string"); // this is ok, we're returning an Own type which transfers ownership
	result
}

// struct cannot outlive the reference passed into part
struct ImportantExcerpt<'a> {
	part: &'a str, // has reference to string in the part field. To make this work, lifetime annotation needs to be specified
}

fn main() {
	let novel: String = String::from("Call me Ishmael. Some years ago...");
	let first_sentence: &str = novel.split('.').next().expect("Could not find");
	let i: ImportantExcerpt = ImportantExcerpt {
		part: first_sentence,
	};
	// after first_sentence has gone out of scope we will get an error
}

fn first_word<'a>(s: &'a str) -> &'a str { // if you remove <'a> the function still works fine. Compiler CAN determinisitcally infer lifetime annotations
	let bytes: &[u8] = s.as_bytes();
	for (i: usize, &item: u8) in bytes.iter().enumerate() {
		if item == b' ' {
			return &s[0..i]
		}
	}
	&s[..]
}

impl<'a> ImportantExcerpt<'a> {
	fn return_part(&self, announcement: &str) -> &str { // you do not need to specify lifetimes since &self is passed in. output has lifetime of &self
		println!("Attention please: {}", announcement);
		self.part
	}
}

fn main() {
	let s: &'static str = "I have a static lifetime";
}

/*
1. Each parameter that is a reference gets its own lifetime parameter
2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters
3. If there are multiple input lifetime parameters, but one of them is &self or &mut self the lifetime of self is assigned to all output lifetime params
*/