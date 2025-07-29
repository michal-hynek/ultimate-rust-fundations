use futures::TryStreamExt;
use sqlx::{prelude::FromRow, Pool, Row, Sqlite};

#[derive(Debug, FromRow)]
struct Message {
    id: i64,
    message: String,
}

async fn raw_query(pool: &Pool<Sqlite>) -> anyhow::Result<()> {
    let messages  = sqlx::query("select id, message from messages")
        .map(|row: sqlx::sqlite::SqliteRow| {
            let id: u32 = row.get(0);
            let message: String = row.get(1);
            (id, message)
        })
        .fetch_all(pool)
        .await?;

    for (id, message) in messages {
        println!("{id}: {message}");
    }

    Ok(())
}

async fn typed_query(pool: &Pool<Sqlite>) -> anyhow::Result<()> {
    let messages: Vec<Message> = sqlx::query_as("select id, message from messages")
        .fetch_all(pool)
        .await?;

    println!("{messages:#?}");

    Ok(())
}

async fn stream_query(pool: &Pool<Sqlite>) -> anyhow::Result<()> {
    let mut message_stream = sqlx::query_as::<_, Message>("select id, message from messages")
        .fetch(pool);

    while let Some(message) = message_stream.try_next().await? {
        println!("{message:#?}");
    }

    Ok(())
}

async fn update_query(id: i64, message: &str, pool: &Pool<Sqlite>) -> anyhow::Result<()> {
    let updated_entries = sqlx::query("update messages set message = ? where id = ?")
        .bind(message)
        .bind(id)
        .execute(pool)
        .await?;

    println!("updated entries {}", updated_entries.rows_affected());
    println!("updated message {id}");

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?; 
    let db_url = std::env::var("DATABASE_URL")?;
    let pool = sqlx::SqlitePool::connect(&db_url).await?;

    tracing_subscriber::fmt::init();

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    update_query(1, "First Message", &pool).await?;

    raw_query(&pool).await?;
    typed_query(&pool).await?;
    stream_query(&pool).await?;

    Ok(())
}
