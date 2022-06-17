use std::{env, str::FromStr};

use log::info;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use strum_macros::{Display, EnumString};

use crate::forecasts::ui::range::Range;

pub struct NewForecast {
    pub name: String,
    pub forecast_type: ForecastType,
}

#[derive(Debug, PartialEq, EnumString, Display)]
pub enum ForecastType {
    Date,
}

pub struct SavedForecast {
    pub id: i64,
    pub name: String,
    pub forecast_type: ForecastType,
    pub data: Option<RangeForecast>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RangeForecast {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub ranges: Option<Vec<Range>>,
}

impl SavedForecast {
    pub fn from(new_forecast: NewForecast, id: i64) -> SavedForecast {
        // This is a mapping from NewForecast to SavedForecast.
        SavedForecast {
            id,
            name: new_forecast.name.clone(),
            forecast_type: new_forecast.forecast_type,
            data: None,
        }
    }
}

#[derive(Clone)]
pub struct Database {
    pub pool: SqlitePool,
}

impl Database {
    pub async fn new() -> Self {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not defined!");
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect(&database_url)
            .await
            .unwrap();
        Self { pool }
    }

    pub async fn migrate(&self) -> anyhow::Result<()> {
        sqlx::migrate!().run(&self.pool).await?;
        Ok(())
    }

    pub async fn find(&self) -> anyhow::Result<(Vec<SavedForecast>)> {
        let recs = sqlx::query!(
            r#"
SELECT id, name, forecastType
FROM forecast
ORDER BY id
        "#
        )
        .fetch_all(&self.pool)
        .await?;
        let mut forecasts = Vec::new();
        for rec in recs {
            forecasts.push(SavedForecast {
                id: rec.id,
                name: rec.name,
                forecast_type: ForecastType::from_str(&rec.forecastType)
                    .expect("Invalid forecast type"),
                data: None,
            })
        }
        Ok(forecasts)
    }

    pub async fn create(&self, forecast: NewForecast) -> anyhow::Result<SavedForecast> {
        let forecast_type = forecast.forecast_type.to_string();
        let id = sqlx::query!(
            r#"
INSERT INTO forecast (name, forecastType)
VALUES (?1, ?2);
        "#,
            forecast.name,
            forecast_type
        )
        .execute(&self.pool)
        .await?
        .last_insert_rowid();
        Ok(SavedForecast::from(forecast, id))
    }

    pub async fn read_by_name(&self, name: String) -> Option<SavedForecast> {
        let rec = sqlx::query!(
            r#"
SELECT id, name, forecastType
FROM forecast
WHERE name = ?1
        "#,
            name
        )
        .fetch_one(&self.pool)
        .await;

        match rec {
            Ok(rec) => Some(SavedForecast {
                id: rec.id,
                name: rec.name,
                forecast_type: ForecastType::from_str(&rec.forecastType)
                    .expect("Invalid forecast type"),
                data: None,
            }),
            Err(e) => match e {
                sqlx::Error::RowNotFound => None,
                _ => panic!("{}", e),
            },
        }
    }

    pub async fn read_by_id(&self, id: i64) -> Option<SavedForecast> {
        let rec = sqlx::query!(
            r#"
SELECT id, name, forecastType
FROM forecast
WHERE id = ?1
        "#,
            id
        )
        .fetch_one(&self.pool)
        .await;

        match rec {
            Ok(rec) => Some(SavedForecast {
                id: rec.id,
                name: rec.name,
                forecast_type: ForecastType::from_str(&rec.forecastType)
                    .expect("Invalid forecast type"),
                data: None,
            }),
            Err(e) => match e {
                sqlx::Error::RowNotFound => None,
                _ => panic!("bad {}", e),
            },
        }
    }

    pub async fn update_data(&self, id: i64, data: RangeForecast) -> anyhow::Result<()> {
        let data_json = serde_json::to_string(&data)?;
        let rec = sqlx::query!(
            r#"
UPDATE forecast
SET data = ?1
WHERE id =?2 
        "#,
            data_json,
            id
        )
        .execute(&self.pool)
        .await;

        match rec {
            Ok(_) => Ok(()),
            Err(e) => match e {
                sqlx::Error::RowNotFound => Err(anyhow::anyhow!(
                    "Updating data but found no forecast with id {}",
                    id
                )),
                _ => panic!("{}", e),
            },
        }
    }

    pub async fn update(&self, forecast: SavedForecast) -> anyhow::Result<()> {
        let rec = sqlx::query!(
            r#"
UPDATE forecast
SET name = ?1
WHERE id = ?2;
        "#,
            forecast.name,
            forecast.id
        )
        .fetch_one(&self.pool)
        .await;

        match rec {
            Ok(_) => Ok(()),
            Err(e) => match e {
                sqlx::Error::RowNotFound => {
                    Err(anyhow::anyhow!("No forecast with id {}", forecast.id))
                }
                _ => panic!("{}", e),
            },
        }
    }

    pub async fn delete(&self, id: i64) -> anyhow::Result<()> {
        let rec = sqlx::query!(
            r#"
DELETE FROM forecast
WHERE id = ?1;
        "#,
            id
        )
        .execute(&self.pool)
        .await;

        match rec {
            Ok(_) => Ok(()),
            Err(e) => match e {
                sqlx::Error::RowNotFound => Err(anyhow::anyhow!("No forecast with id {}", id)),
                _ => panic!("{}", e),
            },
        }
    }
}
