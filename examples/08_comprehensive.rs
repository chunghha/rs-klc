//! Example 08: Comprehensive Example
//!
//! This example demonstrates a complete workflow using multiple features of the library.

use rs_klc::LunarSolarConverter;

fn main() {
    let mut converter = LunarSolarConverter::new();
    let current_year = 2022;
    let current_month = 7;
    let current_day = 10;

    println!("=== 8. Comprehensive Example ===");
    println!(
        "Target Date: {}-{:02}-{:02}",
        current_year, current_month, current_day
    );

    if converter.set_solar_date(current_year, current_month, current_day) {
        println!("--------------------------------------------------");
        println!("Solar Date: {}", converter.get_solar_iso_format());
        println!("Lunar Date: {}", converter.get_lunar_iso_format());
        println!("--------------------------------------------------");
        println!("Korean Gapja: {}", converter.get_gapja_string());
        println!("Chinese Gapja: {}", converter.get_chinese_gapja_string());
        println!("--------------------------------------------------");

        if let Some(dow) =
            LunarSolarConverter::get_day_of_week(current_year, current_month, current_day)
        {
            println!("Day of Week: {:?}", dow);
        }

        if let Some(jdn) =
            LunarSolarConverter::get_julian_day_number(current_year, current_month, current_day)
        {
            println!("Julian Day Number: {}", jdn);
        }

        println!(
            "Is Leap Year: {}",
            LunarSolarConverter::is_solar_leap_year(current_year)
        );

        let lunar_year = converter.lunar_year();
        match LunarSolarConverter::get_lunar_intercalary_month(lunar_year) {
            Some(month) => println!("Lunar Year {} has intercalary month: {}", lunar_year, month),
            None => println!("Lunar Year {} has no intercalary month", lunar_year),
        }
    }
}
