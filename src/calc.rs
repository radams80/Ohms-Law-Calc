pub fn calculate_voltage(current: f64, resistance: f64) -> Result<f64, String> {
    Ok(current * resistance)
}

pub fn calculate_current(voltage: f64, resistance: f64) -> Result<f64, String> {
    if resistance == 0.0 {
        Err(String::from("Error: Resistance cannot be zero (division by zero)."))
    } else if resistance < 0.0 {
        Err(String::from("Error: Resistance cannot be negative."))
    } else {
        Ok(voltage / resistance)
    }
}

pub fn calculate_resistance(voltage: f64, current: f64) -> Result<f64, String> {
    if current == 0.0 {
        Err(String::from("Error: Current cannot be zero (division by zero)."))
    } else if current < 0.0 {
        Err(String::from("Error: Current cannot be negative."))
    } else {
        Ok(voltage / current)
    }
}