use futures::TryStreamExt;
use sqlx::mysql::MySqlPoolOptions;
use tracing;
use tracing_subscriber::EnvFilter;
use xp_connx::consts::DOCKER_URI;

#[tokio::main] // Requires the `attributes` feature of `async-std`
// or #[tokio::main]
// or #[actix_web::main]
async fn main() -> Result<(), sqlx::Error> {
    tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env())
                             .init();
    let pool = MySqlPoolOptions::new().max_connections(5)
                                      .connect(DOCKER_URI)
                                      .await?;
    let num_rows = 3;
    let single_row: (i32, String, String) =
        sqlx::query_as("SELECT id, subdomain, name FROM accounts LIMIT ?").bind(num_rows)
                                                                          .fetch_one(&pool)
                                                                          .await?;

    tracing::info!(row_oath = ?single_row);

    let mut rows = sqlx::query("SELECT id, subdomain, name FROM accounts LIMIT ?").bind(num_rows)
                                                                                  .fetch(&pool);

    while let Some(row) = rows.try_next().await? {
        let row = row;
        tracing::info!(?row);
    }
    // tracing::warn!(?rows);

    Ok(())
}
