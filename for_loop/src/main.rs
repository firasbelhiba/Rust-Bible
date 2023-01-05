fn for_loop() {
    for x in 1..11 {
        if x == 3 {
            continue;
        }

        if x == 8 {
            break;
        }

        println!("x {}", x);
    }
}

fn for_loop_function() {
    for (position, y) in (30..41).enumerate() {
        println!("{} , {}", position, y);
    }
}

fn main() {
    //   for_loop();
    for_loop_function();
}
