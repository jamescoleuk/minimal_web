use actix_web::{web, HttpResponse, Result};
use askama::Template;
use log::info;

use crate::{db::SavedForecast, AppData};

// #[derive(Template)]
// #[template(path = "forecasts/_list.html")]
// struct ListTemplate<'a> {
//     forecasts: &'a Vec<String>,
// }

#[derive(Template)]
#[template(path = "forecasts/list.html")]
struct ListTemplate<'a> {
    forecasts: &'a Vec<SavedForecast>,
}

pub async fn list(app_data: web::Data<AppData>) -> Result<HttpResponse> {
    let results = app_data.database.find().await.unwrap();
    let s = ListTemplate {
        forecasts: &results,
    }
    .render()
    .unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
