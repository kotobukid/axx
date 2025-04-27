mod auth;
mod api;
mod handlers;
mod todo;

use std::collections::HashMap;
use axum::{
    routing::{
        get,
        post,
    },
    Router,
    http::{StatusCode},
    response::{IntoResponse},
    Json,
    Extension,
    extract::{Path},
};

use std::net::SocketAddr;
use std::env;
use std::sync::Arc;
use std::str::FromStr;
use config::{Config};
use serde::Deserialize;
use std::convert::TryFrom;
use sqlx::PgPool;
use dotenv::dotenv;
use tower_http::services::ServeDir;

use crate::auth::{create_user, login_form, login_process};
use crate::handlers::{all_todo, create_todo, delete_todo, find_todo, get_api_router, update_todo};
use crate::todo::{TodoRepositoryForMemory, TodoRepository, TodoRepositoryForDb};

#[derive(Debug, Deserialize)]
struct Settings {
    host: String,
    port: u16,
}

impl TryFrom<Config> for Settings {
    type Error = config::ConfigError;

    fn try_from(config: Config) -> Result<Self, Self::Error> {
        config.try_into()
    }
}

#[tokio::main]
async fn main() {
    let settings = Config::builder()
        // Add in `./Settings.toml`
        .add_source(config::File::with_name("config"))
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        // .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();

    let s = settings
        .try_deserialize::<HashMap<String, String>>()
        .unwrap();


    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());

    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();

    let port = 3000;
    dotenv().ok();

    let database_url = &env::var("DATABASE_URL").expect("undefined [DATABASE_URL]");

    let pool = PgPool::connect(database_url)
        .await
        .expect(&format!("fail connect database, url is [{}]", database_url));

    let repository = TodoRepositoryForDb::new(pool);
    let app = create_app(repository.into());

    let addr = SocketAddr::from_str(&format!("{}:{}", s.get("host").unwrap(), port))
        .expect("Invalid address");

    println!("http://{addr}");
    tracing::debug!("listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

fn create_app<T: TodoRepository>(repository: Arc<T>) -> Router {
    let todo_repository = Arc::clone(&repository);

    let api_router = get_api_router(repository);

    // 個別にルーティングを外付け
    let api_router_c = api_router.clone().route("/json_sample", get(api_sample));

    Router::new()
        .route("/users", post(create_user))
        .nest("/api", api_router_c)
        .route("/login/", get(login_form).post(login_process))
        .nest_service("/", ServeDir::new("../vite-project/dist"))
}

// async fn root() -> impl IntoResponse {
//     (StatusCode::OK, Html(r#"<link rel="stylesheet" href="/static/style.less" /><body><h1>Hello, world!</h1><br /><a href="/login/">Login</a></body>"#))
// }

async fn api_sample() -> impl IntoResponse {
    let user = auth::User {
        id: 1338,
        username: "Taro".into(),
    };
    (StatusCode::OK, Json(user))
}

#[cfg(test)]
mod test {
    use super::*;
    use axum::{
        body::Body,
        http::{header, Method, Request},
    };
    use tower::ServiceExt;
    use crate::auth::User;

    macro_rules! assert_starts_with {
        ($str:expr, $prefix:expr $(,)?) => {{
            let s = $str;
            let p = $prefix;
            assert!(
                s.starts_with(p),
                "assertion failed: `{} starts with {}`",
                s,
                p
            );
        }};
    }
    #[tokio::test]
    async fn should_return_hello_world() {
        let repository = TodoRepositoryForMemory::new();
        let req = Request::builder().uri("/").body(Body::empty()).unwrap();
        let res = create_app(repository.into()).oneshot(req).await.unwrap();

        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();

        let body: String = String::from_utf8(bytes.to_vec()).unwrap();
        // assert_starts_with!(body, r#"<!DOCTYPE html>"#);
        // assert_starts_with!(body, r#"<!DOCTYPE html>\n<html lang=\"ja\">\n<head>\n    <meta charset=\"UTF-8\"/>\n    <link rel=\"icon\" type=\"image/svg+xml\" href=\"/vite.svg\"/>\n    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\"/>\n    <title>Vite + Vue + TS</title>"#);
    }

    #[tokio::test]
    async fn should_return_user_data() {
        let req = Request::builder().uri("/users").method(Method::POST)
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(r#"{ "username": "田中 太郎"}"#)).unwrap();

        let repository = TodoRepositoryForMemory::new();

        let res = create_app(repository.into()).oneshot(req).await.unwrap();

        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();

        let body: String = String::from_utf8(bytes.to_vec()).unwrap();

        let user: User = serde_json::from_str(&body).expect("cannot convert User instance.");
        assert_eq!(user, User {
            id: 1337,
            username: "田中 太郎".to_string(),
        })
    }
}