mod auth;
mod api;
mod handlers;

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

use tower_http::services::ServeDir;
use crate::api::user_list;
use crate::auth::{create_user, CreateTodo, login_form, login_process, TodoRepository, TodoRepositoryForMemory, UpdateTodo};
use crate::handlers::{all_todo, create_todo, delete_todo, find_todo, update_todo};

#[tokio::main]
async fn main() {
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());

    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();


    let repository = TodoRepositoryForMemory::new();
    let app = create_app(repository.into());

    let addr: SocketAddr = SocketAddr::from(([192, 168, 33, 10], 3000));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn create_app<T: TodoRepository>(repository: Arc<T>) -> Router {
    let todo_repository = Arc::clone(&repository);

    Router::new()
        // .route("/", get(root))
        .route("/users", post(create_user))
        // .route("/api/json_sample", get(api_sample))
        .nest("/api", Router::new()
            .route("/json_sample", get(api_sample))
            .route("/users", get(user_list))
            .route("/todos",
                   post({
                       let todo_repository = Arc::clone(&todo_repository);
                       move |payload: Json<CreateTodo>| create_todo(payload, Extension(todo_repository))
                   })
                       .get({
                           let todo_repository = Arc::clone(&todo_repository);
                           move || all_todo(Extension(todo_repository))
                       }))
            .route("/todos/:id", get({
                let todo_repository = Arc::clone(&todo_repository);
                move |id: Path<i32>| find_todo(id, Extension(todo_repository))
            })
                .delete({
                    let todo_repository = Arc::clone(&todo_repository);
                    move |id: Path<i32>| delete_todo(id, Extension(todo_repository))
                })
                .patch({
                    let todo_repository = Arc::clone(&todo_repository);
                    move |id: Path<i32>, payload: Json<UpdateTodo>| update_todo(id, payload, Extension(todo_repository))
                }),
            ),
              // ).layer(Extension(Arc::new(todo_repository))),
        )
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

    // #[tokio::test]
    // async fn should_return_hello_world() {
    //     let req = Request::builder().uri("/").body(Body::empty()).unwrap();
    //     let res = create_app().oneshot(req).await.unwrap();
    //
    //     let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
    //
    //     let body: String = String::from_utf8(bytes.to_vec()).unwrap();
    //     assert_eq!(body, r#"<link rel="stylesheet" href="/static/style.less" /><body><h1>Hello, world!</h1><br /><a href="/login/">Login</a></body>"#);
    // }

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