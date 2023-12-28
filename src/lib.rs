use sea_orm::DatabaseConnection;

pub mod errors;
mod middleware;
pub mod model;
pub mod routes;
pub mod service;
pub mod socket;

#[derive(Debug, Clone)]
pub struct CtxUser {
    pub id: u64,
    pub uid: String,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
    pub user: Option<CtxUser>,
}
