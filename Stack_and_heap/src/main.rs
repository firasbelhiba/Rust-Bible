#![allow(dead_code)]

use std::mem;

struct Point {
    x: f64,
    y: f64,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

fn stack_and_heap() {
    let p1 = origin();
    let p2 = Box::new(origin);

    println!("P1 takes up {} bytes", mem::size_of_val(&p1)); // On the stack
    println!("P2 takes up {} bytes", mem::size_of_val(&p2)); // On the heap

    let p3 = *p2; // This is the value of p2
    println!("{}", p3().y);
}

fn main() {
    stack_and_heap()
}
