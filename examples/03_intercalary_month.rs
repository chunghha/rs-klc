//! Example 03: Intercalary Month Handling
//!
//! This example demonstrates how to check for intercalary months in a lunar year
//! and how to convert a date in an intercalary month.

use rs_klc::LunarSolarConverter;

fn main() {
    let mut converter = LunarSolarConverter::new();

    println!("=== 3. Intercalary Month Handling ===");

    // 1. Check if a lunar year has an intercalary month
    let check_year = 2023; // 2023 has an intercalary 2nd month
    match LunarSolarConverter::get_lunar_intercalary_month(check_year) {
        Some(month) => println!(
            "Lunar Year {} has an intercalary month: {}",
            check_year, month
        ),
        None => println!(
            "Lunar Year {} does not have an intercalary month.",
            check_year
        ),
    }

    // 2. Convert an intercalary month date
    // 2023-02-15 (Lunar Intercalary)
    let lunar_year = 2023;
    let lunar_month = 2;
    let lunar_day = 15;
    let is_intercalation = true;

    println!(
        "\nConverting Lunar: {}-{:02}-{:02} (Intercalary: {})",
        lunar_year, lunar_month, lunar_day, is_intercalation
    );

    if converter.set_lunar_date(lunar_year, lunar_month, lunar_day, is_intercalation) {
        println!("Resulting Solar Date: {}", converter.get_solar_iso_format());
    } else {
        println!("Invalid lunar date!");
    }
}
