use chrono::{Duration, NaiveDate};
use log::info;

use self::range::Range;

pub mod create_ranges;
pub mod list;
pub mod range;

pub fn get_ranges(start_date: NaiveDate, end_date: NaiveDate, range_values: &[i32]) -> Vec<Range> {
    let days_in_range = end_date.signed_duration_since(start_date).num_days();
    let number_of_ranges = 5;
    let range_size = days_in_range / number_of_ranges;
    let mut ranges: Vec<Range> = Vec::new();
    let mut range_start_date = start_date;
    let total: i32 = range_values.iter().sum();

    for index in 0..number_of_ranges {
        let range_end_date = range_start_date
            .checked_add_signed(Duration::days(range_size))
            .unwrap();
        let days = (range_end_date - range_start_date).num_days();
        let this_value = *range_values.get(index as usize).unwrap_or(&0);
        let can_floor = total > 100 && this_value > 0;
        let can_ceil = total < 100;
        info!("{}, {}, {}, {}", total, this_value, can_floor, can_ceil);
        let label = format!("{range_start_date} - {range_end_date} ({days} days)").to_string();
        ranges.push(Range {
            start: range_start_date,
            end: range_end_date,
            label,
            value: this_value,
            can_floor,
            can_ceil,
        });
        range_start_date = range_end_date
            .checked_add_signed(Duration::days(1))
            .unwrap();
    }
    ranges
}
