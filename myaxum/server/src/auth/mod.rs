use axum::{
    response::{Html, IntoResponse},
    extract::Form,
    Json,
    http::{StatusCode},
};
use serde::{Deserialize, Serialize};

pub async fn create_user(Json(payload): Json<CreateUser>) -> impl IntoResponse {
    let user = User {
        id: 1337,
        username: payload.username,
    };

    (StatusCode::CREATED, Json(user))
}

pub async fn login_form() -> impl IntoResponse {
    (StatusCode::OK, Html(r#"<form method="post" action="">
    <label><span>ID:</span><input type="text" name="login_id" /></label>
    <br /><label><span>Password:</span><input type="password" name="password"/ ></label>
    <br /><input type="submit" name="submit" value="SUBMIT" />
    </form>"#))
}

#[derive(Deserialize, Debug)]
pub struct LoginInfo {
    login_id: String,
    password: String,
}

pub async fn login_process(Form(payload): Form<LoginInfo>) -> impl IntoResponse {
    println!("ID: {} / Pass: {}", payload.login_id, payload.password);
    (StatusCode::OK, Html("OK"))
}

#[derive(Deserialize)]
pub struct CreateUser {
    username: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct User {
    pub id: u64,
    pub username: String,
}
