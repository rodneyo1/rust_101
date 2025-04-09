use chrono::{Datelike, NaiveDate};
pub use chrono::Weekday as wd; // Re-export for external use

pub fn middle_day(year: usize) -> Option<wd> {
    let year_i32 = year as i32;  // Convert to i32 for chrono functions
    let is_leap = NaiveDate::from_ymd_opt(year_i32, 2, 29).is_some();
    let total_days = if is_leap { 366 } else { 365 };

    if total_days % 2 == 1 {
        let middle_day_num = total_days / 2 + 1;
        let date = NaiveDate::from_yo_opt(year_i32, middle_day_num).unwrap();
        Some(date.weekday())
    } else {
        None
    }
}