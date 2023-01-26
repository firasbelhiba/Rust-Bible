#[allow(dead_code)]
#[allow(unused_variables)]
use std::mem;

fn main() {
    //let a: u8 = 255; // u = unsigned , 8 bits , 0 - 255 
    //print!("a = {}", a); // a is immutable variable ( That means it cannot be changed)

    // u = unsigned (positif from 0 to 2^N - 1 )
    // i = signed (That means it can take a negative value)
    // i ranges from -128 to 127 (minus one because we are counting the zero)
    //let mut b: i8 = 127; // mut = mutable variable
    //print!("b = {} before", b); // b is immutable variable
    //b = 120;
    //print!("b = {} after", b);

    // Rust figures out by himself how much to allocate space in memory for variables
    //let mut c = 123456789; // i32 = 32 bits = 4 bytes
    //print!("c = {} is {} bytes \n", c, mem::size_of_val(&c));
    //c = -5;
    //print!("c = {} \n", c);

    //usize isize
    // let z: isize = 123; // by size we mean how much bytes does take
    // let size_of_z = mem::size_of_val(&z);
    // print!(
    //     "z = {}, and it takes {} and  this is a {}bit OS",
    //     z,
    //     size_of_z,
    //     size_of_z * 8
    // );
    
    // difference between a character and a letter :
    // A charachter can be a letter or ponctuation or signs
    //let d: &str = "a";
    //println!("d = {},   and it takes {} bytes", d, mem::size_of_val(&d));

    // f32 f64 which are standardized by IEEE-754
    // They are all signed (They can be negative or positive)
    // The default type of floating point numbers is f64
    //let e: f32 = 2.6;
    //print!("e = {}, and it takes {} bytes", e, mem::size_of_val(&e));

    // Boolean has 2 values ( which are true or false ). So the expected behaviour is to allocate only 1 bit for the variable.
    // Unfortunately, it does not. The default allocation is a whole byte. Which is kind of a waste of memory.
    //let g: bool = false; // true
    //print!("g = {}, and it takes {} bytes", g, mem::size_of_val(&g));
}
