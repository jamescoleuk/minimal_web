use actix_web::{web, HttpResponse, Result};
use chrono::NaiveDate;
use serde::Deserialize;

use crate::AppData;

use super::ui::{forecast::EditPath, range::create_ranges};

#[derive(Deserialize)]
pub struct GenerateRangeQuery {
    start_date: NaiveDate,
    end_date: NaiveDate,
}

pub async fn generate_ranges(
    path: web::Path<EditPath>,
    query: web::Query<GenerateRangeQuery>,
    app_data: web::Data<AppData>,
) -> Result<HttpResponse> {
    match create_ranges(
        &app_data.database,
        path.id,
        query.start_date,
        query.end_date,
    )
    .await
    {
        Ok(_) => {
            // if we're successful we want to get the
            Ok(HttpResponse::TemporaryRedirect()
                .append_header(("location", format!("/forecast/{}", path.id)))
                .finish())
        }
        Err(e) => Ok(HttpResponse::NotFound().body(e.to_string())),
    }
}
