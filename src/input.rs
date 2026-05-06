use std::io;
// input handling section

pub fn get_menu_choice() -> u32 {
    println!("Choose what to calculate:");
    println!("1. Voltage (V = I x R)");
    println!("2. Current (I = V / R)");
    println!("3. Resistance (R = V / I)");
    println!("Enter 1, 2, or 3:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().parse::<u32>().unwrap_or(0)
}

pub fn get_number(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid number. Please try again."),
        }
    }
}