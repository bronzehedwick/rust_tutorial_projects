use std::io;

fn main() {
    println!("Convert Fahrenheit to Celsius:");

    let mut temperature = String::new();
    io::stdin().read_line(&mut temperature)
        .expect("Failed to read line");


    let temperature: i32 = match temperature.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    println!("Celsius is: {}", (temperature - 32) * 5 / 9);
}
