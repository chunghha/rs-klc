#![doc = include_str!("../README.md")]

//! ## Usage
//! Add this to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! korean-lunar-calendar = "0.1" // Check crates.io for the latest version
//! ```
//!
//! ## Examples
//! ```rust
//! use korean_lunar_calendar::{LunarSolarConverter, DayOfWeek};
//!
//! fn main() {
//!     let mut converter = LunarSolarConverter::new();
//!
//!     // Set a solar date (e.g., 2022-07-10)
//!     if converter.set_solar_date(2022, 7, 10) {
//!         println!("Solar: {}", converter.get_solar_iso_format());
//!         println!("Lunar: {}", converter.get_lunar_iso_format());
//!         println!("Gapja: {}", converter.get_gapja_string());
//!
//!         // Get day of week
//!         if let Some(dow) = LunarSolarConverter::get_day_of_week(2022, 7, 10) {
//!             println!("Day of Week: {:?}", dow);
//!         }
//!
//!         // Check solar leap year
//!         let is_solar_leap = LunarSolarConverter::is_solar_leap_year(2022);
//!         println!("Solar Year 2022 Leap: {}", is_solar_leap);
//!
//!         // Check lunar intercalary month for the corresponding lunar year
//!         let lunar_year = converter.lunar_year();
//!         if let Some(intercalary_month) = LunarSolarConverter::get_lunar_intercalary_month(lunar_year) {
//!             println!("Lunar Year {} has intercalary month: {}", lunar_year, intercalary_month);
//!         } else {
//!             println!("Lunar Year {} has no intercalary month.", lunar_year);
//!         }
//!     } else {
//!         println!("Invalid solar date");
//!     }
//! }
//! ```

// Declare the module where the implementation resides
pub mod klc;

// Re-export the main struct and enum for easier access
pub use klc::{DayOfWeek, LunarSolarConverter};
