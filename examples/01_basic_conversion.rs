//! Example 01: Basic Solar to Lunar Conversion
//!
//! This example shows how to convert a solar date to a lunar date.

use rs_klc::LunarSolarConverter;

fn main() {
    let mut converter = LunarSolarConverter::new();

    // 2022-07-10 (Solar) -> 2022-06-12 (Lunar)
    let solar_year = 2022;
    let solar_month = 7;
    let solar_day = 10;

    println!("=== 1. Basic Solar to Lunar Conversion ===");
    println!(
        "Input Solar Date: {}-{:02}-{:02}",
        solar_year, solar_month, solar_day
    );

    if converter.set_solar_date(solar_year, solar_month, solar_day) {
        println!("Converted Lunar Date: {}", converter.get_lunar_iso_format());
        println!("Lunar Year: {}", converter.lunar_year());
        println!("Lunar Month: {}", converter.lunar_month());
        println!("Lunar Day: {}", converter.lunar_day());
        println!("Is Intercalary: {}", converter.is_intercalation());
    } else {
        println!("Failed to convert solar date.");
    }
}
