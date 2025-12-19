//! Example 02: Lunar to Solar Conversion
//!
//! This example shows how to convert a lunar date to a solar date.

use rs_klc::LunarSolarConverter;

fn main() {
    let mut converter = LunarSolarConverter::new();

    // 2022-06-12 (Lunar) -> 2022-07-10 (Solar)
    let lunar_year = 2022;
    let lunar_month = 6;
    let lunar_day = 12;
    let is_intercalation = false;

    println!("=== 2. Lunar to Solar Conversion ===");
    println!(
        "Input Lunar Date: {}-{:02}-{:02} (Intercalary: {})",
        lunar_year, lunar_month, lunar_day, is_intercalation
    );

    if converter.set_lunar_date(lunar_year, lunar_month, lunar_day, is_intercalation) {
        println!("Converted Solar Date: {}", converter.get_solar_iso_format());
        println!("Solar Year: {}", converter.solar_year());
        println!("Solar Month: {}", converter.solar_month());
        println!("Solar Day: {}", converter.solar_day());
    } else {
        println!("Failed to convert lunar date.");
    }
}
