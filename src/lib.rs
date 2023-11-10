pub mod routes;
pub mod db;
pub mod model;

#[derive(Debug, Clone)]
pub struct Context {
    pub user: model::user::User,
}