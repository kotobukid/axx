use std::collections::HashMap;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use axum::{
    extract::Extension,
    response::{Html, IntoResponse},
    extract::Form,
    Json,
    http::{StatusCode},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use anyhow::Context;

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

#[derive(Debug, Error)]
enum RepositoryError {
    #[error("NotFound, id is {0}")]
    NotFound(i32),
}

pub trait TodoRepository: Clone + Send + Sync + 'static {
    fn create(&self, payload: CreateTodo) -> Todo;
    fn find(&self, id: i32) -> Option<Todo>;
    fn all(&self) -> Vec<Todo>;
    fn update(&self, id: i32, payload: UpdateTodo) -> anyhow::Result<Todo>;
    fn delete(&self, id: i32) -> anyhow::Result<()>;
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Todo {
    id: i32,
    text: String,
    completed: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct CreateTodo {
    text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct UpdateTodo {
    text: Option<String>,
    completed: Option<bool>,
}

impl Todo {
    pub fn new(id: i32, text: String) -> Self {
        Self {
            id,
            text,
            completed: false,
        }
    }
}

type TodoDatas = HashMap<i32, Todo>;

#[derive(Debug, Clone)]
pub struct TodoRepositoryForMemory {
    store: Arc<RwLock<TodoDatas>>,
}

impl TodoRepositoryForMemory {
    pub fn new() -> Self {
        TodoRepositoryForMemory {
            store: Arc::default(),
        }
    }

    fn write_store_ref(&self) -> RwLockWriteGuard<TodoDatas> {
        self.store.write().unwrap()
    }

    fn read_store_ref(&self) -> RwLockReadGuard<TodoDatas> {
        self.store.read().unwrap()
    }
}

impl TodoRepository for TodoRepositoryForMemory {
    fn create(&self, payload: CreateTodo) -> Todo {
        let mut store = self.write_store_ref();
        let id = (store.len() + 1) as i32;
        let todo = Todo::new(id, payload.text.clone());
        store.insert(id, todo.clone());
        todo
    }

    fn find(&self, id: i32) -> Option<Todo> {
        let store = self.read_store_ref();
        store.get(&id).map(|todo| todo.clone())
    }

    fn all(&self) -> Vec<Todo> {
        let store = self.read_store_ref();
        Vec::from_iter(store.values().map(|todo| todo.clone()))
    }

    fn update(&self, id: i32, payload: UpdateTodo) -> anyhow::Result<Todo> {
        let mut store = self.write_store_ref();
        let todo = store.get(&id).context(RepositoryError::NotFound(id))?;
        let text = payload.text.unwrap_or(todo.text.clone());
        let completed = payload.completed.unwrap_or(todo.completed);
        let todo = Todo {
            id,
            text,
            completed,
        };
        store.insert(id, todo.clone());
        Ok(todo)
    }

    fn delete(&self, id: i32) -> anyhow::Result<()> {
        let mut store = self.read_store_ref();
        store.remove(&id).ok_or(RepositoryError::NotFound(id))?;
        Ok(())
    }
}

pub async fn create_todo<T: TodoRepository>(
    Json(payload): Json<CreateTodo>,
    Extension(repository): Extension<Arc<T>>,
) -> impl IntoResponse {
    let todo = repository.create(payload);

    (StatusCode::CREATED, Json(todo))
}