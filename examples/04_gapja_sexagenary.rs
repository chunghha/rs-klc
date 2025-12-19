//! Example 04: Gapja (Sexagenary Cycle) Calculation
//!
//! This example demonstrates how to get the Korean and Chinese Gapja strings
//! for a given date.

use rs_klc::LunarSolarConverter;

fn main() {
    let mut converter = LunarSolarConverter::new();
    let year = 2022;
    let month = 7;
    let day = 10;

    println!("=== 4. Gapja (Sexagenary Cycle) Calculation ===");

    if converter.set_solar_date(year, month, day) {
        println!("Date: {}-{:02}-{:02}", year, month, day);

        // Korean Gapja
        let gapja_kr = converter.get_gapja_string();
        println!("Korean Gapja: {}", gapja_kr);

        // Chinese Gapja
        let gapja_cn = converter.get_chinese_gapja_string();
        println!("Chinese Gapja: {}", gapja_cn);
    } else {
        println!("Invalid solar date.");
    }
}
