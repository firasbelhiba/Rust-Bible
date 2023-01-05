fn main() {
    // Match statement is like if statement except it can check several cases at the same time
    let country_code = 216;

    let country = match country_code {
        44 => "United Kingdom",
        216 => "Tunisia",
        7 => "Russia",
        1..=100 => "Unkown",
        _ => "Invalid",
    };

    println!("The country with the code {} is {} ", country_code, country);

    let x = false;

    let s = match x {
        true => "Yes",
        false => "no",
    };
}
