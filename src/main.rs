use axum::{
    routing::{get, post},
    Json,
    Router,
    http::StatusCode,
    response::{IntoResponse, Html},
};
use axum::extract::Form;
use std::net::SocketAddr;
use std::env;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());

    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();

    let app = create_app();
    let addr: SocketAddr = SocketAddr::from(([192, 168, 33, 10], 3000));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn create_app() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/users", post(create_user))
        .route("/login/", get(login_form).post(login_process))
}

async fn root() -> impl IntoResponse {
    (StatusCode::OK, Html(r#"<h1>Hello, world</h1><br /><a href="/login/">Login</a>"#))
}

async fn create_user(Json(payload): Json<CreateUser>) -> impl IntoResponse {
    let user = User {
        id: 1337,
        username: payload.username,
    };

    (StatusCode::CREATED, Json(user))
}

async fn login_form() -> impl IntoResponse {
    (StatusCode::OK, Html(r#"<form method="post" action="">
    <label><span>ID:</span><input type="text" name="login_id" /></label>
    <br /><label><span>Password:</span><input type="password" name="password"/ ></label>
    <br /><input type="submit" name="submit" value="SUBMIT" />
    </form>"#))
}

#[derive(Deserialize, Debug)]
struct LoginInfo {
    login_id: String,
    password: String,
}

async fn login_process(Form(payload): Form<LoginInfo>) -> impl IntoResponse {
    println!("{:?}", payload);
    (StatusCode::OK, Html("OK"))
}

#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct User {
    id: u64,
    username: String,
}

#[cfg(test)]
mod test {
    use super::*;
    use axum::{
        body::Body,
        http::{header, Method, Request},
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn should_return_hello_world() {
        let req = Request::builder().uri("/").body(Body::empty()).unwrap();
        let res = create_app().oneshot(req).await.unwrap();

        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();

        let body: String = String::from_utf8(bytes.to_vec()).unwrap();
        assert_eq!(body, "Hello, world");
    }

    #[tokio::test]
    async fn should_return_user_data() {
        let req = Request::builder().uri("/users").method(Method::POST)
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(r#"{ "username": "田中 太郎"}"#)).unwrap();

        let res = create_app().oneshot(req).await.unwrap();

        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();

        let body: String = String::from_utf8(bytes.to_vec()).unwrap();

        let user: User = serde_json::from_str(&body).expect("cannot convert User instance.");
        assert_eq!(user, User {
            id: 1337,
            username: "田中 太郎".to_string(),
        })
    }
}