//! Example 05: Julian Day Number (JDN) Calculation
//!
//! This example demonstrates how to calculate the Julian Day Number for a solar date.

use rs_klc::LunarSolarConverter;

fn main() {
    println!("=== 5. Julian Day Number (JDN) ===");

    let dates = vec![
        (2022, 7, 10),
        (1582, 10, 4),  // Last day of Julian calendar
        (1582, 10, 15), // First day of Gregorian calendar
    ];

    for (year, month, day) in dates {
        match LunarSolarConverter::get_julian_day_number(year, month, day) {
            Some(jdn) => println!("JDN for {}-{:02}-{:02}: {}", year, month, day, jdn),
            None => println!(
                "Could not calculate JDN for {}-{:02}-{:02}",
                year, month, day
            ),
        }
    }
}
