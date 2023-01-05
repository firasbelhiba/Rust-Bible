union Int_Or_Float {
    i: i32,
    f: f32,
}

fn process_value(iof: Int_Or_Float) {
    unsafe {
        match iof {
            Int_Or_Float { i: 42 } => {
                println!("This value is 42")
            }

            Int_Or_Float { f } => {
                println!("This value is {}", f);
            }
        }
    }
}

fn main() {
    let mut iof = Int_Or_Float { i: 123 };
    iof.i = 234;

    let value = unsafe { iof.i };
    println!("{}", value);

    process_value(Int_Or_Float { f: 42.5 });
}
