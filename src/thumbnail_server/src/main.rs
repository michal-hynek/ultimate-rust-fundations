use std::{net::SocketAddr, path::Path};

use axum::{body::Body, extract::Multipart, http::{header, HeaderMap}, response::{Html, IntoResponse}, routing::{get, post, Router}, Extension, Form, Json};
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Pool, Row, Sqlite};
use tokio::task::spawn_blocking;
use tokio_util::io::ReaderStream;

async fn index_page() -> Html<String> {
    let path = Path::new("src/index.html");
    let content = tokio::fs::read_to_string(path).await.unwrap();
    Html(content)
}

async fn uploader(Extension(pool): Extension<Pool<Sqlite>>, mut multipart: Multipart) -> Html<String> {
    let mut image= None;
    let mut tags = None;

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        match name.as_str() {
            "image" => image = Some(data.to_vec()),
            "tags" => tags = Some(String::from_utf8(data.to_vec()).unwrap()),
            _ => panic!("Unknown field: {name}"),
        }

        println!("{name} is {} bytes", data.len());
    }

    if let (Some(image), Some(tags)) = (image, tags) {
        let new_image_id = insert_image_into_database(&pool, &tags).await.unwrap();
        save_image(new_image_id, &image).await.unwrap();
        spawn_blocking(move || {
            make_thumbnail(new_image_id).unwrap();
        });
    } else {
        panic!("Missing field");
    }

    let path= Path::new("src/redirect.html");
    let content = tokio::fs::read_to_string(path).await.unwrap();
    Html(content)
}

async fn insert_image_into_database(pool: &Pool<Sqlite>, tags: &str) -> anyhow::Result<i64> {
    let row = sqlx::query("insert into images (tags) values (?) returning id")
        .bind(tags)
        .fetch_one(pool)
        .await?;

    Ok(row.get(0))
}

async fn save_image(id: i64, bytes: &[u8]) -> anyhow::Result<()> {
    // Check that the images folder exists and is a directory
    // If it doesn't, create it.
    let base_path = Path::new("images");
    if !base_path.exists() || !base_path.is_dir() {
        tokio::fs::create_dir_all(base_path).await?;
    }

    // Use "join" to create a path to the image file. Join is platform aware,
    // it will handle the differences between Windows and Linux.
    let image_path = base_path.join(format!("{id}.jpg"));
    if image_path.exists() {
        // The file exists. That shouldn't happen.
        anyhow::bail!("File already exists");
    }

    // Write the image to the file
    tokio::fs::write(image_path, bytes).await?;
    Ok(())
}

async fn get_image(axum::extract::Path(id): axum::extract::Path<i64>) -> impl IntoResponse {
    let filename = format!("images/{id}.jpg");
    get_image_data(filename).await
}

async fn get_thumbnail(axum::extract::Path(id): axum::extract::Path<i64>) -> impl IntoResponse {
    let filename = format!("images/{id}_thumb.jpg");
    get_image_data(filename).await
}

async fn get_image_data(filename: String) -> impl IntoResponse {
    let attachment = format!("filename={filename}");
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("image/jpeg"),
    );
    headers.insert(
        header::CONTENT_DISPOSITION,
        header::HeaderValue::from_str(&attachment).unwrap()
    );
    let file = tokio::fs::File::open(&filename).await.unwrap();
    axum::response::Response::builder()
        .header(header::CONTENT_TYPE, header::HeaderValue::from_static("image/jpeg"))
        .header(header::CONTENT_DISPOSITION, header::HeaderValue::from_str(&attachment).unwrap())
        .body(Body::from_stream(ReaderStream::new(file)))
        .unwrap()
}

fn make_thumbnail(id: i64) -> anyhow::Result<()> {
    let image_path = format!("images/{id}.jpg");
    let thumbnail_path = format!("images/{id}_thumb.jpg");
    let image_bytes: Vec<u8> = std::fs::read(image_path)?;
    let image = if let Ok(format) = image::guess_format(&image_bytes) {
        image::load_from_memory_with_format(&image_bytes, format)?
    } else {
        image::load_from_memory(&image_bytes)?
    };
    let thumbnail = image.thumbnail(100, 100);
    thumbnail.save(thumbnail_path)?;
    Ok(())
}

#[derive(Deserialize, Serialize, FromRow, Debug)]
struct ImageRecord {
    id: i64,
    tags: String,
}

async fn list_images(Extension(pool): Extension<sqlx::SqlitePool>) -> Json<Vec<ImageRecord>> {
    sqlx::query_as::<_, ImageRecord>("select id, tags from images order by id")
        .fetch_all(&pool)
        .await
        .unwrap()
        .into()
}

async fn fill_missing_thumbnails(pool: &Pool<Sqlite>) -> anyhow::Result<()> {
    let mut rows = sqlx::query("SELECT id FROM images")
        .fetch(pool);

    while let Some(row) = rows.try_next().await? {
        let id = row.get::<i64, _>(0);
        let thumbnail_path = format!("images/{id}_thumb.jpg");
        if !Path::new(&thumbnail_path).exists() {
            spawn_blocking(move || {
                make_thumbnail(id)
            }).await??;
        }
    }

    Ok(())
}

#[derive(Deserialize)]
struct Search {
    tags: String
}

async fn search_images(Extension(pool): Extension<sqlx::SqlitePool>, Form(form): Form<Search>) -> Html<String> {
    let tag = format!("%{}%", form.tags);

    let rows = sqlx::query_as::<_, ImageRecord>("select id, tags from images where tags like ? order by id")
        .bind(tag)
        .fetch_all(&pool)
        .await
        .unwrap();

    let mut results = String::new();
    for row in rows {
        results.push_str(&format!("<a href=\"/images/{}\"><img src='/thumbnails/{}' /></a><br />", row.id, row.id));
    }

    let path = std::path::Path::new("src/search.html");
    let mut content = tokio::fs::read_to_string(path).await.unwrap();
    content = content.replace("{results}", &results);

    Html(content)
}

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    dotenv::dotenv()?;
    let db_url = std::env::var("DATABASE_URL")?;
    let pool = sqlx::SqlitePool::connect(&db_url).await?;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    fill_missing_thumbnails(&pool).await.unwrap();

    let app = Router::new()
        .route("/", get(index_page))
        .route("/images", get(list_images))
        .route("/images/{id}", get(get_image))
        .route("/thumbnails/{id}", get(get_thumbnail))
        .route("/upload", post(uploader))
        .route("/search", post(search_images))
        .layer(Extension(pool));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
