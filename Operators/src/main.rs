fn main() {
    // Arithmetic

    let mut a = 5 + 5 * 2;
    println!("a: {}", a);

    // Rust does not support the "--" and "++"
    // If we want to increase the value of a number we need to do like this:
    a = a + 1;
    // Or this way
    a += 1; // same goes for minus,times,divides and modulo
    println!("a: {}", a);

    // % or modulo means the remainder of the division in Rust
    println!("The remainder of {} / {} = {}", a, 10, (a % 3));

    // Unfortunately, there is no power operator in Rust
    let a_squared = i16::pow(5, 2);
    let a_cubed = i64::pow(a, 3);
    println!("a_squared: {}", a_squared);
    println!("a_cubed: {}", a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("{} cubed is {} and {}^{}", b, b_cubed, b, b_to_pi);


    // Bitwise operators 
    // In Rust, bitwise operators are only availble for integers 
    // | is the OR 
    // & is the AND 
    // ^ is the XOR  
    // ! is the NOR
}
