//mod input;
//mod calc;
//mod output;

fn main() {
    println!("Ohm's Law Calculator - Week 1 Checkpoint");

    let choice = input::get_menu_choice();

    match choice {
        1 => {
            let current = input::get_number("Enter current (I) in amps: ");
            let resistance = input::get_number("Enter resistance (R) in ohms: ");
            let result = calc::calculate_voltage(current, resistance);
            output::display_result("Voltage", result, "V");
        }
        2 => {
            let voltage = input::get_number("Enter voltage (V) in volts: ");
            let resistance = input::get_number("Enter resistance (R) in ohms: ");
            let result = calc::calculate_current(voltage, resistance);
            output::display_result("Current", result, "A");
        }
        3 => {
            let voltage = input::get_number("Enter voltage (V) in volts: ");
            let current = input::get_number("Enter current (I) in amps: ");
            let result = calc::calculate_resistance(voltage, current);
            output::display_result("Resistance", result, "Ohms");
        }
        _ => {
            output::display_error("Invalid menu choice.");
        }
    }
}