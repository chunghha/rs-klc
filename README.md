# korean-lunar-calendar: Korean Lunar-Solar Calendar Converter

A Rust library for converting between Korean Lunar dates and Gregorian Solar dates. It also calculates the traditional Korean and Chinese Gapja (sexagenary cycle) names for the year, month, and day.

This library is based on the data and algorithms originally implemented in JavaScript by [usingsky](https://github.com/usingsky/korean-lunar-calendar) and adapted for Rust.

## Features

*   **Lunar to Solar Conversion**: Convert a Korean Lunar date (including intercalary months) to the corresponding Gregorian Solar date.
*   **Solar to Lunar Conversion**: Convert a Gregorian Solar date to the corresponding Korean Lunar date.
*   **Gapja Calculation**: Calculate the Korean (`갑자`) and Chinese (`干支`) sexagenary cycle names for the year, month, and day of a given date.
*   **Julian Day Number (JDN)**: Calculate the JDN for a solar date.
*   **Day of the Week**: Determine the day of the week (Monday-Sunday) for a solar date.
*   **Solar Leap Year Check**: Check if a given solar year is a leap year (handling Julian/Gregorian rules).
*   **Lunar Intercalary Month Check**: Check if a given lunar year contains an intercalary month (윤달) and which month it is.
*   **Date Validation**: Checks if the provided dates are within the supported range and handles historical anomalies like the Gregorian calendar reform gap in October 1582.
*   **Intercalary Month Handling**: Correctly identifies and processes intercalary (leap) months in the Lunar calendar during conversions.
*   **ISO Formatting**: Provides simple ISO 8601 format output (`YYYY-MM-DD`) for both Lunar and Solar dates.

See the [Examples](#examples) section for code demonstrating these features.

## Supported Date Range

*   **Lunar**: 1391-01-01 to 2050-11-18
*   **Solar**: 1391-02-05 to 2050-12-31

*(Note: Dates are based on the Korean time zone (KST) implicitly via the underlying data/algorithm design.)*

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
rs-klc = "0.1" # Check crates.io for the latest version
```

Then use the library like this:

```rust
use rs_klc::{LunarSolarConverter, DayOfWeek};

let mut converter = LunarSolarConverter::new();

// Set a Solar date and get Lunar info
if converter.set_solar_date(2022, 7, 10) {
    println!("Solar: {}", converter.get_solar_iso_format()); // Output: 2022-07-10
    println!("Lunar: {}", converter.get_lunar_iso_format()); // Output: 2022-06-12
    println!("Gapja: {}", converter.get_gapja_string()); // Output: 임인년 정미월 갑자일

    // Get day of week for the solar date
    if let Some(dow) = LunarSolarConverter::get_day_of_week(2022, 7, 10) {
        println!("Day of Week: {:?}", dow); // Output: Sunday
    }
} else {
    println!("Invalid solar date");
}

println!("--- Check other features ---");

// Check solar leap year
let solar_year = 2024;
let is_solar_leap = LunarSolarConverter::is_solar_leap_year(solar_year);
println!("Solar Year {} Leap: {}", solar_year, is_solar_leap); // Output: true

// Check lunar intercalary month
let lunar_year = 2023;
if let Some(intercalary_month) = LunarSolarConverter::get_lunar_intercalary_month(lunar_year) {
    println!("Lunar Year {} has intercalary month: {}", lunar_year, intercalary_month); // Output: 2 (윤2월)
} else {
    println!("Lunar Year {} has no intercalary month.", lunar_year);
}

// Calculate JDN
if let Some(jdn) = LunarSolarConverter::get_julian_day_number(2022, 7, 10) {
    println!("JDN for 2022-07-10: {}", jdn); // Output: 2459771
}
```

## Examples
 
The `examples/` directory contains several examples demonstrating different features of the library. You can run them using `cargo` or `task`:

*   `01_basic_conversion`: Basic Solar <-> Lunar conversion
*   `02_lunar_to_solar`: Lunar to Solar conversion
*   `03_intercalary_month`: Intercalary month handling
*   `04_gapja_sexagenary`: Korean/Chinese Gapja calculation
*   `05_julian_day_number`: Julian Day Number calculation
*   `06_leap_year`: Solar leap year check
*   `07_day_of_week`: Day of week calculation
*   `08_comprehensive`: Comprehensive feature demo

Run all examples:
```bash
task run-examples
```

## Building and Testing

This project uses `cargo` for building and testing, and `task` (Taskfile) for convenience.

*   **Build**: 
    ```bash
    cargo build
    # or
    task build
    ```
*   **Test**: 
    ```bash
    cargo test
    # or
    task test
    ```
*   **Run Examples**: 
    ```bash
    cargo run --example 01_basic_conversion
    # or
    task example1
    ```
*   **Lint**: 
    ```bash
    cargo clippy
    # or
    task lint
    ```

## License

This project is licensed under the MIT License. See the `LICENSE` file for details. 