use std::io; 
	
fn fib(array: &mut[i32; 45]){
	let mut a: i32 = 1; 
	let mut b: i32 = 1;
	array[0] = 1; 
	for idx in 1..45 {
		// i am assuming n > 1
		array[idx as usize] = b; 
		println!("a[{idx}]={b}");
		b = a + b; 
		a = b - a;
	}
}	

fn bin_search(a:&[i32],x:i32) -> i32 {
	let mut l: usize = 0; 
	let mut r: usize = a.len()-1;
	let mut m: usize = 0; 
	if a[l] > x || a[r] < x {return -1}
	while l<=r{
		println!("{l} {r} {x}");
		m = (l + r)/2;
		if a[m]==x {
			break;
		}
		else if a[m]>x {
			r = m-1; 
		} 
		else {
			l = m + 1;
		}
	}
	if a[m]==x {m as i32} else {-1}
}

fn main() {
	let mut a: [i32;45] = [0;45];	
	fib(&mut a);
	loop {
		println!("Enter a number:");
		let mut x: String = String::new();
		io::stdin()
			.read_line(&mut x)
			.expect("failed to read line!");
		let x: i32 = match x.trim().parse() {
			Ok(num) => {num},
			Err(_) => {println!("Number likhna mc!");continue;}
		}; 
		let ans = bin_search(&a,x); 
		if ans == -1 {
			println!("{x} is not a fibonacci number!");
		}
		else {println!("{x} is the {ans}th fibonacci number");}
	}
}
