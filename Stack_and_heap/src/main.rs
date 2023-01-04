#![allow(dead_code)]

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

    println!("P1 takes up {} bytes", mem::size_of_val(&p1));
}

fn main() {
    stack_and_heap()
}
