mod part02;
use std::io::prelude::*;
use std::io;

impl Minumum for f32 {
    fn min(self, b: Self) -> Self {
        if self < b { self } else { b }
    }
}

/*

fn read_vec_generic() -> Vec<T:Num> {
    let mut vec : Vec<T:Num> = Vec::<T:Num>::new();

    let stdin = io::stdin();
    println!("Enter a list of numbers, one per line. End with Ctrl-D (Linux) or Ctrl-Z (Windows).");

    for line in stdin.lock().lines() {
        let line = line.unwrap();

        match line.trim().parse::<T:Num>() {
            Ok(num) => {
                vec.push(num);
            },
            Err(_) => {
                println!("I'm an error message");
            }
        }
    }
    vec
}*/

fn read_vec() -> Vec<i32> {
    let mut vec : Vec<i32> = Vec::<i32>::new();

    let stdin = io::stdin();
    println!("Enter a list of numbers, one per line. End with Ctrl-D (Linux) or Ctrl-Z (Windows).");

    for line in stdin.lock().lines() {
        let line = line.unwrap();

        match line.trim().parse::<i32>() {
            Ok(num) => {
                vec.push(num);
            },
            Err(_) => {
                println!("I'm an error message");
            }
        }
    }
    vec
}

use part02::{SomethingOrNothing, IntegerOrNothing, Something, Nothing, Minumum, vec_min};

pub trait Print {
    fn print(self);
}

impl Print for i32 {
    fn print(self) {
        println!("The number is {}", self);
    }
}

impl<T:Print> SomethingOrNothing<T> {
    fn print2(self) {
        match self {
            Nothing => println!("Nothing!"),
            Something(n) => n.print(),
        }
    }
}

pub fn main() {
    let vec = read_vec();
    let min = vec_min(vec);
    min.print2();
}