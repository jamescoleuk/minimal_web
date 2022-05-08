use actix_files as fs;
use actix_web::{middleware, web, App, HttpResponse, HttpServer, Result};
use askama::Template;
use chrono::{Duration, NaiveDate};
use log::info;
use std::collections::HashMap;

#[derive(Template)]
#[template(path = "forecast.html")]
struct ForecastTemplate<'a> {
    forecast_name: &'a str,
}

#[derive(Template)]
#[template(path = "list.html")]
struct ListTemplate<'a> {
    forecasts: &'a Vec<String>,
}

struct Range {
    start: NaiveDate,
    end: NaiveDate,
    label: String, // TODO: this should be (<start> - <end> (<days> days))
    value: i32,
}

#[derive(Template)]
#[template(path = "ranges.html")]
struct RangesTemplate<'a> {
    ranges: &'a Vec<Range>,
    start_date: NaiveDate,
    end_date: NaiveDate,
    total: &'a i32,
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index;

async fn index(query: web::Query<HashMap<String, String>>) -> Result<HttpResponse> {
    let s = if let Some(forecast_name) = query.get("forecast_name") {
        ForecastTemplate { forecast_name }.render().unwrap()
    } else {
        Index.render().unwrap()
    };
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

async fn list() -> Result<HttpResponse> {
    let s = ListTemplate {
        forecasts: &vec![
            "forecast 1".to_string(),
            "forecast 2".to_string(),
            "forecast 3".to_string(),
        ],
    }
    .render()
    .unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

async fn update_ranges(query: web::Query<HashMap<String, String>>) -> Result<HttpResponse> {
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

fn get_ranges(start_date: NaiveDate, end_date: NaiveDate, range_values: &[i32]) -> Vec<Range> {
    let days_in_range = end_date.signed_duration_since(start_date).num_days();

    let number_of_ranges = 5;

    let range_size = days_in_range / number_of_ranges;
    let mut ranges: Vec<Range> = Vec::new();

    let mut range_start_date = start_date;

    for index in 0..number_of_ranges {
        let range_end_date = range_start_date
            .checked_add_signed(Duration::days(range_size))
            .unwrap();
        let days = (range_end_date - range_start_date).num_days();
        ranges.push(Range {
            start: range_start_date,
            end: range_end_date,
            label: format!("{range_start_date} - {range_end_date} ({days} days)").to_string(),
            value: *range_values.get(index as usize).unwrap_or(&0),
        });
        range_start_date = range_end_date
            .checked_add_signed(Duration::days(1))
            .unwrap();
    }
    ranges
}

async fn generate_ranges(query: web::Query<HashMap<String, String>>) -> Result<HttpResponse> {
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/list").route(web::get().to(list)))
            .service(web::resource("/generate_ranges").route(web::get().to(generate_ranges)))
            .service(web::resource("/update_ranges").route(web::get().to(update_ranges)))
            .service(fs::Files::new("/static", "./static").show_files_listing())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
