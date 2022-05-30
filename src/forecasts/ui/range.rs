use actix_web::{web, HttpRequest, HttpResponse, Result};
use askama::Template;
use chrono::{Duration, NaiveDate};
use log::info;
use serde::Deserialize;
use std::collections::HashMap;

use super::get_ranges;

pub struct Range {
    pub(crate) start: NaiveDate,
    pub(crate) end: NaiveDate,
    pub(crate) label: String,
    pub(crate) value: i32,
    pub(crate) can_ceil: bool,
    pub(crate) can_floor: bool,
}

#[derive(Template)]
#[template(path = "forecasts/_range.html")]
pub struct RangesTemplate<'a> {
    pub(crate) ranges: &'a Vec<Range>,
    pub(crate) start_date: NaiveDate,
    pub(crate) end_date: NaiveDate,
    pub(crate) total: &'a i32,
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
