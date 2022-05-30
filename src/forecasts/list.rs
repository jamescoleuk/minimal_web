use actix_web::{web, HttpResponse, Result};
use askama::Template;

use crate::AppData;

#[derive(Template)]
#[template(path = "forecasts/management/_list.html")]
struct ListTemplate<'a> {
    forecasts: &'a Vec<String>,
}

/// Get the list of forecasts
pub async fn list(app_data: web::Data<AppData>) -> Result<HttpResponse> {
    let results = app_data.database.find().await.unwrap();
    let names: Vec<String> = results.iter().map(|f| f.name.clone()).collect();
    let s = ListTemplate { forecasts: &names }.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
