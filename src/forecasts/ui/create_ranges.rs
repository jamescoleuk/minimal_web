use actix_web::{web, HttpRequest, HttpResponse, Result};
use askama::Template;
use chrono::{Duration, NaiveDate};
use log::info;
use serde::Deserialize;
use std::collections::HashMap;

use super::{get_ranges, range::RangesTemplate};

pub async fn create(query: web::Query<HashMap<String, String>>) -> Result<HttpResponse> {
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
