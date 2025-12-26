//! # Conversion Library
//!
//! A simple engineering unit conversion library for common measurements.
//!
//! This library provides functions for converting between different units of measurement
//! commonly used in engineering projects: length, temperature, pressure, and mass.
//!
//! # Examples
//!
//! ```
//! use conversion_lib::length;
//! use conversion_lib::temperature;
//!
//! // Convert meters to feet
//! let feet = length::meters_to_feet(100.0);
//! assert_eq!(feet, 328.084);
//!
//! // Convert Celsius to Fahrenheit
//! let fahrenheit = temperature::celsius_to_fahrenheit(25.0);
//! assert_eq!(fahrenheit, 77.0);
//! ```

/// Length conversion functions
pub mod length {
    /// Converts meters to feet
    ///
    /// # Arguments
    ///
    /// * `meters` - Length in meters
    ///
    /// # Returns
    ///
    /// Length in feet
    ///
    /// # Examples
    ///
    /// ```
    /// use conversion_lib::length;
    ///
    /// let feet = length::meters_to_feet(10.0);
    /// assert_eq!(feet, 32.808399999999995);
    /// ```
    pub fn meters_to_feet(meters: f64) -> f64 {
        meters * 3.28084
    }

    /// Converts feet to meters
    ///
    /// # Arguments
    ///
    /// * `feet` - Length in feet
    ///
    /// # Returns
    ///
    /// Length in meters
    ///
    /// # Examples
    ///
    /// ```
    /// use conversion_lib::length;
    ///
    /// let meters = length::feet_to_meters(100.0);
    /// assert_eq!(meters, 30.48);
    /// ```
    pub fn feet_to_meters(feet: f64) -> f64 {
        feet * 0.3048
    }

    /// Converts kilometers to miles
    ///
    /// # Arguments
    ///
    /// * `km` - Distance in kilometers
    ///
    /// # Returns
    ///
    /// Distance in miles
    pub fn km_to_miles(km: f64) -> f64 {
        km * 0.621371
    }

    /// Converts miles to kilometers
    pub fn miles_to_km(miles: f64) -> f64 {
        miles * 1.60934
    }
}

/// Temperature conversion functions
pub mod temperature {
    /// Converts Celsius to Fahrenheit
    ///
    /// # Arguments
    ///
    /// * `celsius` - Temperature in Celsius
    ///
    /// # Returns
    ///
    /// Temperature in Fahrenheit
    ///
    /// # Examples
    ///
    /// ```
    /// use conversion_lib::temperature;
    ///
    /// let fahrenheit = temperature::celsius_to_fahrenheit(0.0);
    /// assert_eq!(fahrenheit, 32.0);
    /// ```
    pub fn celsius_to_fahrenheit(celsius: f64) -> f64 {
        (celsius * 9.0 / 5.0) + 32.0
    }

    /// Converts Fahrenheit to Celsius
    pub fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
        (fahrenheit - 32.0) * 5.0 / 9.0
    }

    /// Converts Celsius to Kelvin
    pub fn celsius_to_kelvin(celsius: f64) -> f64 {
        celsius + 273.15
    }

    /// Converts Kelvin to Celsius
    pub fn kelvin_to_celsius(kelvin: f64) -> f64 {
        kelvin - 273.15
    }
}

/// Pressure conversion functions
pub mod pressure {
    /// Converts bar to PSI (pounds per square inch)
    ///
    /// # Arguments
    ///
    /// * `bar` - Pressure in bar
    ///
    /// # Returns
    ///
    /// Pressure in PSI
    pub fn bar_to_psi(bar: f64) -> f64 {
        bar * 14.5038
    }

    /// Converts PSI to bar
    pub fn psi_to_bar(psi: f64) -> f64 {
        psi * 0.0689476
    }

    /// Converts Pascal to bar
    pub fn pascal_to_bar(pascal: f64) -> f64 {
        pascal / 100000.0
    }

    /// Converts bar to Pascal
    pub fn bar_to_pascal(bar: f64) -> f64 {
        bar * 100000.0
    }
}

/// Mass conversion functions
pub mod mass {
    /// Converts kilograms to pounds
    pub fn kg_to_pounds(kg: f64) -> f64 {
        kg * 2.20462
    }

    /// Converts pounds to kilograms
    pub fn pounds_to_kg(pounds: f64) -> f64 {
        pounds * 0.453592
    }

    /// Converts metric tons (tonnes) to short tons
    pub fn tonnes_to_tons(tonnes: f64) -> f64 {
        tonnes * 1.10231
    }

    /// Converts short tons to metric tons (tonnes)
    pub fn tons_to_tonnes(tons: f64) -> f64 {
        tons * 0.907185
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_conversions() {
        // Test meters to feet
        let result = length::meters_to_feet(1.0);
        assert!((result - 3.28084).abs() < 0.0001);

        // Test round trip conversion
        let meters = 100.0;
        let feet = length::meters_to_feet(meters);
        let back_to_meters = length::feet_to_meters(feet);
        assert!((meters - back_to_meters).abs() < 0.0001);
    }

    #[test]
    fn test_temperature_conversions() {
        // Test freezing point
        assert_eq!(temperature::celsius_to_fahrenheit(0.0), 32.0);
        
        // Test boiling point
        assert_eq!(temperature::celsius_to_fahrenheit(100.0), 212.0);

        // Test Kelvin
        assert_eq!(temperature::celsius_to_kelvin(0.0), 273.15);
    }

    #[test]
    fn test_pressure_conversions() {
        let bar = 1.0;
        let psi = pressure::bar_to_psi(bar);
        let back_to_bar = pressure::psi_to_bar(psi);
        assert!((bar - back_to_bar).abs() < 0.0001);
    }

    #[test]
    fn test_mass_conversions() {
        let kg = 100.0;
        let pounds = mass::kg_to_pounds(kg);
        let back_to_kg = mass::pounds_to_kg(pounds);
        assert!((kg - back_to_kg).abs() < 0.0001);
    }
}
