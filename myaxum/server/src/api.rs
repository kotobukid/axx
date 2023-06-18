use crate::auth::User;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct Users {
    users: Vec<User>,
}

pub async fn user_list() -> impl IntoResponse {
    let users = Users {
        users: vec![
            User { id: 0, username: "Taro".into() },
            User { id: 1, username: "Jiro".into() },
            User { id: 2, username: "Saburo".into() },
        ]
    };

    (StatusCode::OK, Json(users))
}