use actix_files as fs;
use actix_web::{middleware, web, App, HttpResponse, HttpServer, Result};
use askama::Template;
use db::Database;
use forecasts::ui::{
    forecast::{create, edit, CreateForecastTemplate},
    list::list,
    range::{ceiling, floor, update_ranges},
};
use log::info;
use std::collections::HashMap;

use crate::{db::NewForecast, forecasts::routes::generate_ranges};

mod db;
mod forecasts;

// async fn index(
//     query: web::Query<HashMap<String, String>>,
//     app_data: web::Data<AppData>,
// ) -> Result<HttpResponse> {
//     let s = if let Some(forecast_name) = query.get("forecast_name") {
//         let database = &app_data.database;
//         let forecast = match database.read_by_name(forecast_name.to_string()).await {
//             Some(forecast) => {
//                 info!("Found forecast {}", forecast.id);
//                 forecast
//             }
//             None => {
//                 info!("Forecast does not exist -- creating {}", forecast_name);
//                 let new_forecast = NewForecast {
//                     name: forecast_name.to_string(),
//                 };
//                 app_data.database.create(new_forecast).await.unwrap()
//             }
//         };
//         ForecastTemplate {
//             forecast_name: forecast.name.as_str(),
//         }
//         .render()
//         .unwrap()
//     } else {
//         CreateForecastTemplate.render().unwrap()
//     };
//     Ok(HttpResponse::Ok().content_type("text/html").body(s))
// }

async fn index(
    query: web::Query<HashMap<String, String>>,
    app_data: web::Data<AppData>,
) -> Result<HttpResponse> {
    Ok(HttpResponse::TemporaryRedirect()
        .append_header(("location", "/forecast/list"))
        .finish())
}

#[derive(Clone)]
pub struct AppData {
    pub database: Database,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // std::env::set_var("RUST_LOG", "info,sqlx=error,actix=error");
    std::env::set_var("LOG_LEVEL", "info");
    info!("Starting up");
    println!("Starting up print");
    env_logger::init();

    let database = Database::new().await;
    database.migrate().await.unwrap();
    let app_data = AppData { database };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_data.clone()))
            .wrap(middleware::Logger::default())
            // We reason about browser paths and async paths in different ways.
            // For example it's pretty obvious that in a browser we might go to
            // /forecasts/1 to load forecast with id=1. There are a limited number
            // of these, and they are how we structure our pages. These are different
            // enough from the paritals we return using htmx that we want to keep them
            // separate. E.g. /adjust_range does not belong with /forecast/1. So we need
            // to separate these paths within the actix code, but we also need to have
            // a distinct naming convention. We're going to go with conventional naming
            // for the page paths, e.g.:
            //   /forecasts/{id}
            // And something different for the async paths, e.g.:
            //   /forecasts/_list
            // There's got to be a lot of dicipline to keep this naming straight. I'm not
            // a fan of having to do this but I've not worked out an alternative yet.
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/forecast/create").route(web::get().to(create)))
            .service(web::resource("/forecast/list").route(web::get().to(list))) // TODO: Call this mini-list
            .service(web::resource("/forecast/{id}").route(web::get().to(edit)))
            .service(
                web::resource("/forecast/{id}/_generate_ranges")
                    .route(web::get().to(generate_ranges)),
            )
            .service(
                web::resource("/forecast/_generate_ranges").route(web::get().to(generate_ranges)),
            )
            .service(web::resource("/forecast/_update_ranges").route(web::get().to(update_ranges)))
            .service(web::resource("/forecast/_ceiling_range").route(web::post().to(ceiling)))
            .service(web::resource("/forecast/_floor_range").route(web::post().to(floor)))
            .service(fs::Files::new("/static", "./static").show_files_listing())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
