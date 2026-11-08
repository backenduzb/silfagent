use anyhow::{Context, Result};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;

use migration::{Migrator, MigratorTrait};

pub async fn connect(database_url: &str) -> Result<DatabaseConnection> {
    let mut options = ConnectOptions::new(database_url);

    options
        .max_connections(10)
        .min_connections(1)
        .connect_timeout(Duration::from_secs(10))
        .acquire_timeout(Duration::from_secs(10))
        .sqlx_logging(false);

    let db = Database::connect(options)
        .await
        .context("failed to connect to PostgreSQL")?;
    Migrator::up(&db, None)
        .await
        .context("failed to run database migrations")?;

    Ok(db)
}

pub use connect as psqlconnect;
