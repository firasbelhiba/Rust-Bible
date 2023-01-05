fn if_statement() {
    let temp: u8 = 3;
    if temp > 30 {
        println!("The temperature is hot today");
    } else if temp < 10 {
        println!("The temperature is cold today");
    } else {
        println!("The temperature is fine today");
    }
}

fn main() {
    if_statement();
}
