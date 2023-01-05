fn while_function() {
    let mut x = 1;
    while x < 1000 {
        x += 10;
        if x == 11 {
            // Continue means to continue looping without executing anything under the keyword continue
            continue;
        }
        println!("x {}", x);
    }
}

fn loop_function() {
    let mut y = 1;

    loop
    // loop is like a while true , you need to break of it
    {
        y *= 2;
        println!("y {}", y);

        if y == 1 << 10 {
            break;
        }
    }
}

fn main() {
    while_function();
    loop_function();
}
