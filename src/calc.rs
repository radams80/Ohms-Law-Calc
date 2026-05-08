pub fn calculate_voltage(current: f64, resistance: f64) -> f64 {
    current * resistance
}

pub fn calculate_current(voltage: f64, resistance: f64) -> f64 {
    voltage / resistance
}

pub fn calculate_resistance(voltage: f64, current: f64) -> f64 {
    voltage / current
}