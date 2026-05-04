pub fn display_result(label: &str, result: Result<f64, String>, unit: &str) {
    match result {
        Ok(value) => println!("{} = {:.2} {}", label, value, unit),
        Err(message) => println!("Error: {}", message),
    }
}

pub fn display_error(message: &str) {
    println!("Error: {}", message);
}