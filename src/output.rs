pub fn display_result(label: &str, value: f64, unit: &str) {
    println!("{}: {:.2} {}", label, value, unit);
}

pub fn display_error(message: &str) {
    println!("Error: {}", message);
}
