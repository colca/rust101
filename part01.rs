use std::io::{self, Write};

enum NumberOrNothing {
	Number(i32),
	Nothing
}

use self::NumberOrNothing::{Number, Nothing};

fn vec_min(v:Vec<i32>) -> NumberOrNothing {

	fn min_i32(a: i32, b: i32) -> i32 {
		if a > b {b} else {a}
	}

	let mut min = Nothing;
	for e in v {
		min = Number(match min {
			Nothing => e, 
			Number(n) => min_i32(e, n)
		});
	}
	min
}

fn vec_sum(v:Vec<i32>) -> NumberOrNothing {
	let mut sum = Nothing;
	fn sum_i32(a: i32, b: i32) -> i32 {
		a + b
	}

	for e in v {
		sum = Number(match sum {
			Nothing => e,
			Number(n) => sum_i32(e, n)
		});
	}
	sum
}

fn vec_print(v:Vec<i32>) {
	println!("\nPrinting vector content:");
	for e in v {
		println!("The element is: {}", e);
	}
}

impl NumberOrNothing {
	fn print(self) {
		match self {
			Nothing => println!("The number is :<nothing>"),
			Number(n) => println!("The number is: {}", n),
		};
	}
}

fn read_vec() -> Vec<i32> {
	vec![18, 5, 23, 55, 9, 27]
}

pub fn main() {
	let vec = read_vec();
	let min = vec_min(vec.clone());
	min.print();
	let sum = vec_sum(vec.clone());
	sum.print();
	vec_print(vec.clone());
   	io::stdout().flush().unwrap();

}
