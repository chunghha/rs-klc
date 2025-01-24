mod klc;

use klc::LunarSolarConverter;

fn main() {
    let mut converter = LunarSolarConverter::new();
    converter.set_solar_date(2022, 7, 10);
    let lunar_gapja = converter.get_gapja_string();
    let chinese_gapja = converter.get_chinese_gapja_string();

    println!("{}", lunar_gapja);
    println!("{}", chinese_gapja);

    converter.set_lunar_date(2022, 6, 12, false);
    let solar = converter.get_solar_iso_format();

    println!("{}", solar);

    converter.set_solar_date(2022, 7, 10);
    let lunar = converter.get_lunar_iso_format();

    println!("{}", lunar);

    // Example of using the JDN function
    if let Some(jdn) = LunarSolarConverter::get_julian_day_number(2022, 7, 10) {
        println!("Julian Day Number for 2022-07-10: {}", jdn);
    } else {
        println!("Could not calculate JDN for 2022-07-10");
    }

    // Example of getting the day of the week
    if let Some(dow) = LunarSolarConverter::get_day_of_week(2022, 7, 10) {
        println!("Day of the week for 2022-07-10: {:?}", dow);
    } else {
        println!("Could not calculate day of week for 2022-07-10");
    }

    // Example: Check if Solar year is leap year
    let is_leap = LunarSolarConverter::is_solar_leap_year(2022);
    println!(
        "Solar Year 2022 is a: {}",
        if is_leap {
            "윤년 (Leap year)"
        } else {
            "평년 (Common year)"
        }
    );

    // Example: Check for Lunar intercalary month
    // Note: We need the lunar year calculated by the converter for this check
    let lunar_year_for_check = converter.lunar_year(); // Get the lunar year corresponding to the solar date
    match LunarSolarConverter::get_lunar_intercalary_month(lunar_year_for_check) {
        Some(month) => println!(
            "Lunar Year {} has: 윤달 (Intercalary month: {})",
            lunar_year_for_check, month
        ),
        None => println!(
            "Lunar Year {} has: 평달 (No intercalary month)",
            lunar_year_for_check
        ),
    }
}
