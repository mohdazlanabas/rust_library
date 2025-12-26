use conversion_lib::length;
use conversion_lib::temperature;
use conversion_lib::pressure;
use conversion_lib::mass;

fn main() {
    println!("=== Engineering Unit Conversion Demo ===\n");

    // Length conversions
    println!("LENGTH CONVERSIONS:");
    let meters = 100.0;
    let feet = length::meters_to_feet(meters);
    println!("  {} meters = {:.2} feet", meters, feet);
    
    let km = 42.195; // Marathon distance
    let miles = length::km_to_miles(km);
    println!("  {} km (marathon) = {:.2} miles", km, miles);
    
    // Temperature conversions
    println!("\nTEMPERATURE CONVERSIONS:");
    let celsius = 25.0;
    let fahrenheit = temperature::celsius_to_fahrenheit(celsius);
    let kelvin = temperature::celsius_to_kelvin(celsius);
    println!("  {}°C = {:.2}°F = {:.2}K", celsius, fahrenheit, kelvin);
    
    let boiling = 100.0;
    let boiling_f = temperature::celsius_to_fahrenheit(boiling);
    println!("  Boiling point: {}°C = {:.2}°F", boiling, boiling_f);
    
    // Pressure conversions
    println!("\nPRESSURE CONVERSIONS:");
    let bar = 2.5;
    let psi = pressure::bar_to_psi(bar);
    let pascal = pressure::bar_to_pascal(bar);
    println!("  {} bar = {:.2} PSI = {:.0} Pa", bar, psi, pascal);
    
    // Mass conversions
    println!("\nMASS CONVERSIONS:");
    let kg = 75.0;
    let pounds = mass::kg_to_pounds(kg);
    println!("  {} kg = {:.2} pounds", kg, pounds);
    
    let tonnes = 500.0; // Typical waste per day in AD projects
    let tons = mass::tonnes_to_tons(tonnes);
    println!("  {} tonnes/day = {:.2} tons/day", tonnes, tons);
    
    println!("\n=== Practical Engineering Example ===");
    println!("AD Facility Specifications:");
    let daily_waste_tonnes = 500.0;
    let daily_waste_tons = mass::tonnes_to_tons(daily_waste_tonnes);
    let operating_temp_c = 55.0; // Thermophilic digestion
    let operating_temp_f = temperature::celsius_to_fahrenheit(operating_temp_c);
    let operating_pressure_bar = 1.5;
    let operating_pressure_psi = pressure::bar_to_psi(operating_pressure_bar);
    
    println!("  Capacity: {} tonnes/day ({:.2} tons/day)", daily_waste_tonnes, daily_waste_tons);
    println!("  Operating Temperature: {}°C ({:.2}°F)", operating_temp_c, operating_temp_f);
    println!("  Operating Pressure: {} bar ({:.2} PSI)", operating_pressure_bar, operating_pressure_psi);
}
