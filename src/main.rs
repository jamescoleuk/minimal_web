use actix_files as fs;
use actix_web::{middleware, web, App, HttpResponse, HttpServer, Result};
use askama::Template;
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/list").route(web::get().to(list)))
            .service(fs::Files::new("/static", "./static").show_files_listing())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
