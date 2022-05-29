use std::env;

use sqlx::SqlitePool;

pub struct NewForecast {
    pub name: String,
}

pub struct SavedForecast {
    pub id: i64,
    pub name: String,
}

impl SavedForecast {
    pub fn from(new_forecast: NewForecast, id: i64) -> SavedForecast {
        // This is a mapping from NewForecast to SavedForecast.
        SavedForecast {
            id,
            name: new_forecast.name.clone(),
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
SELECT id, name
FROM forecasts
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
            })
        }
        Ok(forecasts)
    }

    pub async fn create(&self, forecast: NewForecast) -> anyhow::Result<SavedForecast> {
        let id = sqlx::query!(
            r#"
INSERT INTO forecasts (name)
VALUES (?1);
        "#,
            forecast.name
        )
        .execute(&self.pool)
        .await?
        .last_insert_rowid();
        Ok(SavedForecast::from(forecast, id))
    }

    pub async fn read_by_name(&self, name: String) -> Option<SavedForecast> {
        let rec = sqlx::query!(
            r#"
SELECT id, name 
FROM forecasts
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
SELECT id, name 
FROM forecasts
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
            }),
            Err(e) => match e {
                sqlx::Error::RowNotFound => None,
                _ => panic!("{}", e),
            },
        }
    }

    pub async fn update(&self, forecast: SavedForecast) -> anyhow::Result<()> {
        let rec = sqlx::query!(
            r#"
UPDATE forecasts
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
DELETE FROM forecasts 
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
