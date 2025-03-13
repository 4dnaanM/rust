fn takes_ownership(x:String) { 
	let sz:usize = x.len();  
	println!("{x} is of length {sz}");
}
fn takes_and_gives_ownership(x:String) -> (String, usize) {
	let sz:usize = x.len();
	println!("{x} is of length {sz}");
	(x, sz)
}
fn borrows(x:&String) {
	let sz:usize = x.len();
	println!("{x} is of length {sz}");
}
fn borrows_and_changes(x:&mut String) {
	x.push_str("_changed");
	let sz:usize = x.len();
	println!("{x} is of length {sz}");
}
fn first_word(s:&String) -> &str {
	for (i,&item) in s.as_bytes().iter().enumerate() {
		if item == b'_' {
			return &s[0..i]; 
		} 	
	} 
	return &s[..];
}

fn main() {
	// Types of known size can be trivially copied and are stored on the stack
	let _:i32 = 10;  	
	
	// Types of mutable size must be stored in the heap, eg: String
	let s:String = String::from("Hello");
	
	// Assignment from HEAP ALLOCATED var to var drops the from var
	let _x:String = s; 
	// s can't be referenced from now

	// Reassignment of a HEAP ALLOCATED var from val to val drops from val 	
	let _x:String = String::from("Hi");
	// Old value Hello was cleaned from heap	

	// If we need a deep copy, we use .clone() method on a HEAP ALLOCATED var
	let y:String = _x.clone();

	// Types stored in stack should implement Copy trait and not have Drop trait
	
	// Functions reassign params into their scope, so it drops var IF HEAP ASSIGNED
	let z = y.clone();
	takes_ownership(z);
	// z can't be used now
	
	// This can be prevented by borrowing or returning HEAP ASSIGNED variables
	// We can return multiple variables using tuple packing and unpacking
	let (y, _) = takes_and_gives_ownership(y);
	
	// Borrowing a variable is creating a reference to it and passing that
	let ref_y = &y;	
	borrows(ref_y);
	// References are dropped after use and are by default immutable just like variables
	let mut w = y.clone();
	let ref_w = &mut w;
	borrows_and_changes(ref_w);
	
	// Scope of a ref starts from assignment and lasts until the last time its used
	// We can either have ONE MUTABLE or ANY NUMBER OF IMMUTABLE refs in scope
	// let ref2_w = &w will lead to an error if ref_w is used later
	// so will doing borrows(&w) 
	
	// Methods with &self parameter borrow values, and those with self take ownership
	// .iter() borrows values from a collection and returns iterator to each element 
	// .enumerate() returns tuple of each value from a collection wrapped with index
	
	let v = w.clone(); 
	// We can slice a string, using the str type 
	let _part:&str = &v[1..3];
	let _startpart:&str = &v[..5];
	let _endpart:&str = &v[4..];	
	let first:&str = first_word(&v);
	println!("First word of {v} is {first}");
	
	//type of a string literal is actually &str
	// we can init an &str with a reference to or a slice of a String!
	// &[i32] is the type of a slice of an array 


}
