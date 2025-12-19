//! Example 07: Day of Week Calculation
//!
//! This example demonstrates how to determine the day of the week for a solar date.

use rs_klc::LunarSolarConverter;

fn main() {
    println!("=== 7. Day of Week Calculation ===");

    let dates = vec![(2022, 7, 10), (2024, 1, 1)];

    for (year, month, day) in dates {
        match LunarSolarConverter::get_day_of_week(year, month, day) {
            Some(dow) => println!("{}-{:02}-{:02} is {:?}", year, month, day, dow),
            None => println!("Invalid date: {}-{:02}-{:02}", year, month, day),
        }
    }
}
