//In Rust, constants are stored in the read-only memory section of the program,
//rather than on the stack or heap. Because of this, they do not have a specific
//memory address that can be taken. Additionally, constants are guaranteed to be immutable,
//so it would not make sense to take their address, as it would not be possible to change the
//value stored at that address.

const this_a_constant: u8 = 40; // no fixed address

//Static variables in Rust do have a memory address, unlike constants. However, the memory address of a static variable is determined at compile-time, and it is guaranteed to be the same for the entire lifetime of the program.

//Static variables are stored in a specific memory section called the Data segment,
//and they have a fixed location in the memory, unlike stack and heap variables.
//They can be both mutable and immutable, depending on how they are declared,
//so it's possible to take their address and you can access them via the address.

//static variables in Rust can be mutable.
//They are declared using the static keyword and can be marked as mutable using the mut keyword.
static mut z: i32 = 100;

fn main() {
    unsafe {
        println!("{}", z);
    }
}
