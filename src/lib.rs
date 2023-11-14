use sea_orm::DatabaseConnection;

pub mod routes;
pub mod db;
pub mod model;

#[derive(Debug, Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
}