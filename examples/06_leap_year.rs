//! Example 06: Leap Year Check
//!
//! This example demonstrates how to check if a solar year is a leap year.

use rs_klc::LunarSolarConverter;

fn main() {
    println!("=== 6. Solar Leap Year Check ===");

    let years = vec![2020, 2021, 2022, 2023, 2024, 2100, 2000];

    for year in years {
        let is_leap = LunarSolarConverter::is_solar_leap_year(year);
        println!(
            "Year {}: {}",
            year,
            if is_leap { "Leap Year" } else { "Common Year" }
        );
    }
}
