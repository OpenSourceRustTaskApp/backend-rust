use axum::{routing::get, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u64,
    name: String,
    email: String,
}

// ユーザー一覧を返すAPI
pub async fn get_users() -> Json<Vec<User>> {
    // モックデータ
    let users = vec![
        User {
            id: 1,
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        },
        User {
            id: 2,
            name: "Bob".to_string(),
            email: "bob@example.com".to_string(),
        },
    ];

    Json(users)
}

// user モジュールのルーターを作成
pub fn router() -> Router {
    Router::new().route("/users", get(get_users))
}