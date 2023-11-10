use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug, Copy, Clone, sqlx::Type)]
pub enum Delete {
    #[default]
    No = 0,
    Yes = 1,
}

pub mod user;
pub mod conversation;
pub mod message;