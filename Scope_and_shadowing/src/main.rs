fn first() {
    // The variable "a" only exists in the scope of this function ( {} )
    // If we try to println the variable "a" in the second function , that will be impossible
    // Because "a" is not defined in the scope of the second function
    let a: u8 = 5;
}

fn second() {
    let a: u8 = 123;
    {
        let a: u8 = 12;
        println!("This value of a is inside the scope: {} \n", a);
    }
    println!("This value of a is outside of the scope: {} \n", a);
}

fn main() {
    second();
}
