use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
    Json,
    Router,
    routing::{get, post},
};
use std::sync::Arc;
use crate::api::user_list;

use crate::auth::{CreateTodo, TodoRepository, UpdateTodo};

pub async fn create_todo<T: TodoRepository>(
    Json(payload): Json<CreateTodo>,
    Extension(repository): Extension<Arc<T>>,
) -> impl IntoResponse {
    let todo = repository.create(payload);
    (StatusCode::CREATED, Json(todo))
}

pub async fn find_todo<T: TodoRepository>(
    Path(id): Path<i32>,
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = repository.find(id).ok_or(StatusCode::NOT_FOUND)?;
    Ok((StatusCode::OK, Json(todo)))
}

pub async fn all_todo<T: TodoRepository>(
    Extension(repository): Extension<Arc<T>>
) -> impl IntoResponse {
    (StatusCode::OK, Json(repository.all()))
}

pub async fn update_todo<T: TodoRepository>(
    Path(id): Path<i32>,
    Json(payload): Json<UpdateTodo>,
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = repository.update(id, payload)
        .or(Err(StatusCode::NOT_FOUND))?;
    Ok((StatusCode::CREATED, Json(todo)))
}

pub async fn delete_todo<T: TodoRepository>(
    Path(id): Path<i32>,
    Extension(repository): Extension<Arc<T>>,
) -> StatusCode {
    repository.delete(id)
        .map(|_| StatusCode::NO_CONTENT)
        .unwrap_or(StatusCode::NOT_FOUND)
}

pub fn get_api_router<T: TodoRepository>(repository: Arc<T>) -> Router {
    let todo_repository = Arc::clone(&repository);

    Router::new()
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
        )
        // ジェネリクスの疑問解決まではクロージャで回避すべくコメントアウト
        // .layer(Extension(Arc::new(todo_repository))),
}