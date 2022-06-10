use actix_web::{web, HttpResponse, Result};
use askama::Template;
use log::info;
use serde::Deserialize;
use std::collections::HashMap;

use crate::{
    db::{ForecastType, NewForecast},
    AppData,
};

#[derive(Template)]
#[template(path = "forecasts/forecast.html")]
pub struct ForecastTemplate<'a> {
    forecast_name: &'a str,
    forecast_id: &'a str,
}

#[derive(Template)]
#[template(path = "forecasts/create.html")]
pub struct CreateForecastTemplate;

pub async fn create(
    query: web::Query<HashMap<String, String>>,
    app_data: web::Data<AppData>,
) -> Result<HttpResponse> {
    if let Some(forecast_name) = query.get("name") {
        info!("Creating forecast '{}'", forecast_name);
        let database = &app_data.database;
        //TODO: don't do this. Rather let the user know it already exists.
        let forecast = match database.read_by_name(forecast_name.to_string()).await {
            Some(forecast) => {
                info!("Found forecast '{}", forecast.id);
                forecast
            }
            None => {
                info!("Forecast does not exist -- creating {}", forecast_name);
                let new_forecast = NewForecast {
                    name: forecast_name.to_string(),
                    forecast_type: ForecastType::Date, // TODO: pass this in
                };
                app_data.database.create(new_forecast).await.unwrap()
            }
        };
        // Now that we've created the forecast we're going to redirect the user
        // using the new ID.
        let redirect_url = format!("/forecast/{}", forecast.id);
        Ok(HttpResponse::SeeOther()
            .insert_header(("LOCATION", redirect_url))
            .finish())
    } else {
        info!("Inviting user to create a forecast.");
        let body = CreateForecastTemplate.render().unwrap();
        Ok(HttpResponse::Ok().content_type("text/html").body(body))
    }
}

#[derive(Deserialize)]
pub struct EditPath {
    id: i64,
}

pub async fn edit(
    path: web::Path<EditPath>,
    // query: web::Query<HashMap<String, String>>,
    app_data: web::Data<AppData>,
) -> Result<HttpResponse> {
    info!("Editing forecast with id {}", path.id);
    let database = &app_data.database;
    match database.read_by_id(path.id).await {
        Some(forecast) => {
            // TODO: if there's no date then ask for a date
            // TODO: if there's a date then show ranges
            info!("Found forecast {}", forecast.id);
            let body = ForecastTemplate {
                forecast_name: forecast.name.as_str(),
                forecast_id: forecast.id.to_string().as_str(),
            }
            .render()
            .unwrap();
            Ok(HttpResponse::Ok().content_type("text/html").body(body))
        }
        None => Ok(HttpResponse::NotFound().finish()),
    }
}
