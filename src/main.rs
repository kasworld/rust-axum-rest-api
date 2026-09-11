use dotenvy::dotenv;
use sqlx::sqlite::{SqlitePoolOptions, SqlitePool};
use axum::{extract::Extension, routing::get, Json, Router, http::StatusCode, extract::Path};
use tracing::{info, Level};
use tracing_subscriber;
use serde::{Serialize,Deserialize};
use serde_json;

#[derive(Serialize, Deserialize)]
struct Post {
    id: i64,
    user_id: Option<i64>,
    title: String,
    body: String,
}

#[derive(Serialize, Deserialize)]
struct CreatePost {
    title: String,
    body: String,
    user_id: Option<i64>,
}

 #[derive(Serialize, Deserialize)]
struct UpdatePost {
    title: String,
    body: String,
    user_id: Option<i64>,
}

#[derive(Serialize)]
struct Message {
    message: String,
}

#[derive(Serialize, Deserialize)]
struct CreateUser {
    username: String,
    email: String,
}
 
#[derive(Serialize, Deserialize)]
struct User {
    id: i64,
    username: String,
    email: String,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // initialize tracing for logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();
 
    dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = SqlitePoolOptions::new().connect(&url).await?;
    info!("Connected to the database!");
 
    // build our application with a route
    let app = Router::new()
    .route("/users", post(create_user))
    .route("/posts", get(get_posts).post(create_post))
    .route("/posts/{id}", get(get_post).put(update_post).delete(delete_post))
    .layer(Extension(pool));

    // run our app with hyper, listening globally on port 5000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    info!("Server is running on http://0.0.0.0:5000");
    axum::serve(listener, app).await.unwrap();
 
    Ok(())
}
 
// handler for GET /
async fn root() -> &'static str {
    "Hello, world!"
}

async fn get_posts(
    Extension(pool): Extension<SqlitePool>
) -> Result<Json<Vec<Post>>, StatusCode> {
    let posts = sqlx::query_as!(Post, "SELECT id, user_id, title, body FROM posts")
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
 
    Ok(Json(posts))
}

async fn get_post(
    Extension(pool): Extension<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Json<Post>, StatusCode> {
    let post = sqlx::query_as!(
        Post,
        "SELECT id, user_id, title, body FROM posts WHERE id = $1",
        id
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;
 
    Ok(Json(post))
}

 
async fn create_post(
    Extension(pool): Extension<SqlitePool>,
    Json(new_post): Json<CreatePost>,
) -> Result<Json<Post>, StatusCode> {
    let post = sqlx::query_as!(
        Post,
        "INSERT INTO posts (user_id, title, body) VALUES ($1, $2, $3) RETURNING id, title, body, user_id",
        new_post.user_id,
        new_post.title,
        new_post.body
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
 
    Ok(Json(post))
}
 
 
async fn update_post(
    Extension(pool): Extension<SqlitePool>,
    Path(id): Path<i64>,
    Json(updated_post): Json<UpdatePost>,
) -> Result<Json<Post>, StatusCode> {
    let post = sqlx::query_as!(
        Post,
        "UPDATE posts SET title = $1, body = $2, user_id = $3 WHERE id = $4 RETURNING id, user_id, title, body",
        updated_post.title,
        updated_post.body,
        updated_post.user_id,
        id
    )
    .fetch_one(&pool)
    .await;
 
    match post {
        Ok(post) => Ok(Json(post)),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}


async fn delete_post(
    Extension(pool): Extension<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let result = sqlx::query!("DELETE FROM posts WHERE id = $1", id)
        .execute(&pool)
        .await;
 
    match result {
        Ok(_) => Ok(Json(serde_json::json! ({
            "message": "Post deleted successfully"
        }))),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}
 
 
async fn create_user(
    Extension(pool): Extension<SqlitePool>,
    Json(new_user): Json<CreateUser>,
) -> Result<Json<User>, StatusCode> {
    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (username, email) VALUES ($1, $2) RETURNING id, username, email",
        new_user.username,
        new_user.email
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
 
    Ok(Json(user))
}