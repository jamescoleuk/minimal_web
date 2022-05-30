use actix_web::{web, HttpRequest, HttpResponse, Result};
use askama::Template;
use chrono::{Duration, NaiveDate};
use log::info;
use serde::Deserialize;
use std::collections::HashMap;

pub struct Range {
    start: NaiveDate,
    end: NaiveDate,
    label: String,
    value: i32,
    can_ceil: bool,
    can_floor: bool,
}

#[derive(Template)]
#[template(path = "forecasts/_ranges.html")]
struct RangesTemplate<'a> {
    ranges: &'a Vec<Range>,
    start_date: NaiveDate,
    end_date: NaiveDate,
    total: &'a i32,
}

#[derive(Deserialize)]
pub struct RangeFormData {
    start_date: NaiveDate,
    end_date: NaiveDate,
    range_1: i32,
    range_2: i32,
    range_3: i32,
    range_4: i32,
    range_5: i32,
}
pub async fn adjust_range(
    form: web::Form<RangeFormData>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let mut range_values = vec![
        form.range_1,
        form.range_2,
        form.range_3,
        form.range_4,
        form.range_5,
    ];

    let total = range_values.iter().sum::<i32>();
    let mut adjustment = req
        .headers()
        .get("HX-Trigger-Name")
        .expect("HX-Trigger-Name is necessary for me to know what range to apply this to.")
        .to_str()
        .unwrap()
        .split('_');

    let operation = adjustment.next().unwrap();
    let index_of_range_to_adjust = adjustment.next().unwrap().parse::<usize>().unwrap();
    // TODO: index ranges by 0-based index, not 1-based index
    let value_to_adjust = range_values.get(index_of_range_to_adjust - 1).unwrap();

    let new_value = match operation {
        "floor" => {
            let mut floored = value_to_adjust - (total - 100);
            if floored < 0 {
                floored = 0
            }
            floored
        }
        "ceil" => value_to_adjust + (100 - total),
        _ => panic!("Unknown adjustment operation: {}", operation),
    };

    range_values[index_of_range_to_adjust - 1] = new_value;

    let ranges = get_ranges(form.start_date, form.end_date, &range_values);
    let s = RangesTemplate {
        ranges: &ranges,
        start_date: form.start_date,
        end_date: form.end_date,
        total: &range_values.iter().sum(),
    }
    .render()
    .unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn update_ranges(query: web::Query<HashMap<String, String>>) -> Result<HttpResponse> {
    let start_date_str = query.get("start_date").unwrap();
    let end_date_str = query.get("end_date").unwrap();
    let start_date = NaiveDate::parse_from_str(start_date_str, "%Y-%m-%d").unwrap();
    let end_date = NaiveDate::parse_from_str(end_date_str, "%Y-%m-%d").unwrap();

    let range_values = vec![
        query
            .get("range_1")
            .unwrap()
            .parse::<i32>()
            .expect("Range should have a numerical value"),
        query
            .get("range_2")
            .unwrap()
            .parse::<i32>()
            .expect("Range should have a numerical value"),
        query
            .get("range_3")
            .unwrap()
            .parse::<i32>()
            .expect("Range should have a numerical value"),
        query
            .get("range_4")
            .unwrap()
            .parse::<i32>()
            .expect("Range should have a numerical value"),
        query
            .get("range_5")
            .unwrap()
            .parse::<i32>()
            .expect("Range should have a numerical value"),
    ];

    let ranges = get_ranges(start_date, end_date, &range_values);

    let s = RangesTemplate {
        ranges: &ranges,
        start_date,
        end_date,
        total: &range_values.iter().sum(),
    }
    .render()
    .unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

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

pub async fn generate_ranges(query: web::Query<HashMap<String, String>>) -> Result<HttpResponse> {
    let start_date_str = query.get("start_date").unwrap();
    let end_date_str = query.get("end_date").unwrap();

    let start_date = NaiveDate::parse_from_str(start_date_str, "%Y-%m-%d").unwrap();
    let end_date = NaiveDate::parse_from_str(end_date_str, "%Y-%m-%d").unwrap();

    let s = RangesTemplate {
        ranges: &get_ranges(start_date, end_date, &[20, 20, 20, 20, 20]),
        start_date,
        end_date,
        total: &100,
    }
    .render()
    .unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
