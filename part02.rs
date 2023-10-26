

pub enum SomethingOrNothing<T> {
	Something(T),
	Nothing,
}

pub use self::SomethingOrNothing::*;

pub type IntegerOrNothing = SomethingOrNothing<i32>;

impl<T> SomethingOrNothing<T> {
	fn new(o:Option<T>) -> Self {
		match o {
			None => Nothing,
			Some(t) => Something(t)
 		}
	}
	fn to_option(self) -> Option<T> {
		match self {
			Nothing => None,
			Something(t) => Some(t)
		}
	}
}

pub trait Minumum : Copy {
	fn min(self, b:Self) -> Self;
}

pub fn vec_min<T:Minumum> (v: Vec<T>) -> SomethingOrNothing<T> {
	let mut min = Nothing;
	for e in v {
		min = Something(match min {
			Nothing => e,
			Something(t) => e.min(t)
		});
	}
	min
}

impl Minumum for i32 {
	fn min (self, b:Self) -> Self {
		if self < b {self} else {b}
  }
}

impl IntegerOrNothing {
	pub fn print(self) {
		match self {
			Nothing => println!("The number is: <nothong>"),
			Something(n) => println!("The number is: {}", n),
		};
  }
}

fn read_vec() -> Vec<i32> {
	vec![123, 25, -1, 23, 9]
}

pub fn main() {
	let vec = read_vec();
	let min = vec_min(vec);
	min.print();
}
