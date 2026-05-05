pub fn calculate_voltage(current: f64, resistance: f64) -> Result<f64, String> {
    Ok(current * resistance)
}

pub fn calculate_current(voltage: f64, resistance: f64) -> Result<f64, String> {
    if resistance == 0.0 {
        Err(String::from("Resistance cannot be zero."))
    } else {
        Ok(voltage / resistance)
    }
}

pub fn calculate_resistance(voltage: f64, current: f64) -> Result<f64, String> {
    if current == 0.0 {
        Err(String::from("Current cannot be zero."))
    } else {
        Ok(voltage / current)
    }
}