# rs-klc: Korean Lunar-Solar Calendar Converter

A Rust library for converting between Korean Lunar dates and Gregorian Solar dates. It also calculates the traditional Korean and Chinese Gapja (sexagenary cycle) names for the year, month, and day.

This library is based on the data and algorithms originally implemented in JavaScript by [usingsky](https://github.com/usingsky/korean-lunar-calendar) and adapted for Rust.

## Features

*   **Lunar to Solar Conversion**: Convert a Korean Lunar date (including intercalary months) to the corresponding Gregorian Solar date.
*   **Solar to Lunar Conversion**: Convert a Gregorian Solar date to the corresponding Korean Lunar date.
*   **Gapja Calculation**: Calculate the Korean (`갑자`) and Chinese (`干支`) sexagenary cycle names for the year, month, and day of a given date.
*   **Date Validation**: Checks if the provided dates are within the supported range and handles historical anomalies like the Gregorian calendar reform gap in October 1582.
*   **Intercalary Month Handling**: Correctly identifies and processes intercalary (leap) months in the Lunar calendar.
*   **ISO Formatting**: Provides simple ISO 8601 format output (`YYYY-MM-DD`) for both Lunar and Solar dates.

## Supported Date Range

*   **Lunar**: 1391-01-01 to 2050-11-18
*   **Solar**: 1391-02-05 to 2050-12-31

*(Note: Dates are based on the Korean time zone (KST).)*

## Usage

Add `rs-klc` as a dependency to your `Cargo.toml` (if published) or integrate the `src/klc` module into your project.

```rust
// Example from src/main.rs
mod klc;

use klc::LunarSolarConverter;

fn main() {
    let mut converter = LunarSolarConverter::new();

    // Set a Solar date and get Lunar info
    converter.set_solar_date(2022, 7, 10);
    let lunar_gapja = converter.get_gapja_string(); // Get Korean Gapja
    let chinese_gapja = converter.get_chinese_gapja_string(); // Get Chinese Gapja
    let lunar_iso = converter.get_lunar_iso_format(); // Get Lunar date in YYYY-MM-DD

    println!("Solar: 2022-07-10");
    println!("Korean Gapja: {}", lunar_gapja); // Output: 임인년 정미월 갑자일
    println!("Chinese Gapja: {}", chinese_gapja); // Output: 壬寅年 丁未月 甲子日
    println!("Lunar Date: {}", lunar_iso); // Output: 2022-06-12

    println!("---");

    // Set a Lunar date and get Solar info
    converter.set_lunar_date(2022, 6, 12, false); // Year, Month, Day, IsIntercalary
    let solar_iso = converter.get_solar_iso_format(); // Get Solar date in YYYY-MM-DD

    println!("Lunar: 2022-06-12 (Non-intercalary)");
    println!("Solar Date: {}", solar_iso); // Output: 2022-07-10
}
```

## Building and Testing

*   Build: `cargo build`
*   Run example: `cargo run`
*   Run tests: `cargo test`

## License

This project is licensed under the MIT License. See the header comments in `src/klc/mod.rs` for details. 