use std::io;

fn sum(x:i32, y:i32) -> i32 {
	x+y 
}

fn looping_fib(n: isize) -> i32 {
	let mut a:i32  = 0; 
	let mut b:i32 = 1;
	let mut i:isize = 1;
	'fib_loop: loop {
		if i == n {
			break 'fib_loop b
		}
		i = i + 1;
		b = a + b;
		a = b - a;
	}
} 

fn while_fib(n: isize) -> i32{
	let mut a:i32 = 0; 
	let mut b:i32 = 1; 
	let mut i:isize = 1; 
	while i < n {
		i = i + 1; 
		b = a + b; 
		a = b - a; 	
	} 
	b
}

fn for_fib(n:isize) -> i32{
	let mut a:i32 = 0; 
	let mut b:i32 = 1; 
	for _ in 1..n {
		b = a + b; 
		a = b - a; 		
	} 
	b
}

fn main() {
    println!("Hello, world!");
	let x: i32 = 6; 
	println!("The value of x is {x}");
	let x: f32 = (x + 1) as f32;
	let t: bool = true;  
	let mytuple: (i64, f64, bool) = (64, 32.12, false);
	let (i, f, b) = mytuple;
	let a:[String; 2] = [String::from("one"),String::from("two")];
	{
		let x = "seven";
		if t {
			println!("The value of x is {x}");
		}
	}
	println!("The value of x is {x}");
	println!("The values of (i, f, b) are ({i}, {f}, {b})");
	let one: &String = &a[0];
	println!("one is {one}");

	let x:i32 = 1;
	let y:i32 = {
		let t:i32 = 3; 
		t + 1
	};
	println!("The value of x is {x} and y is {y}");
	
	let a = {let b = 3; b};
	println!("a = {a}");
	let s = sum(a,3); 
	println!("s = {s}");
	if a == 3 {
		println!("Condition is true");
	}
	else {
		println!("Condition is false");
	}  
	let num = 10; 
	let other_num = if num == 10 {20} else {10};
	println!("Other num is {other_num}");
	let mut n : String = String::new(); 
	let n: isize = 'take_input: loop {
		println!("Input number for fibonacci: ");
		n.clear(); 
		io::stdin()
			.read_line(&mut n)
			.expect("Failed to read line!");
		match n.trim().parse() {
			Ok(num) => {
				if num <= 0 {
					println!("Input a positive number!");
					continue; 
				} else {
					break 'take_input num
				}
			}, 
			Err(_) => {
				println!("Failed to parse input!"); 
				continue;
			}
		};
	};
	let ans = looping_fib(n);
	println!("looping_fib: {ans}");
	let ans = while_fib(n); 
	println!("while_fib: {ans}");
	let ans = for_fib(n); 
	println!("for fib: {ans}"); 

}
