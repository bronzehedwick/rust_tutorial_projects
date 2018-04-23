use std::io;

fn main() {
    let mut previous_number = 1;
    let mut more_previous_number = 1;
    let mut current_number = 1;

    println!("Enter number:");

    let mut fibonacci_number = String::new();
    io::stdin().read_line(&mut fibonacci_number)
        .expect("Failed to read line");

    let fibonacci_number: u32 = match fibonacci_number.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    if fibonacci_number > 2 {
        let mut index = 0;
        let max = fibonacci_number - 2;
        while index < max {
            current_number = previous_number + more_previous_number;
            more_previous_number = previous_number;
            previous_number = current_number;
            index = index + 1;
        }
    }

    println!("Fibonacci number at position {} is: {}", fibonacci_number, current_number);
}
