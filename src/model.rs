use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Default, Serialize, Deserialize, Debug, Copy, Clone, EnumIter, DeriveActiveEnum, PartialEq,
)]
#[sea_orm(rs_type = "i8", db_type = "TinyInteger")]
pub enum Delete {
    #[default]
    No = 0,
    Yes = 1,
}

pub mod user;
// pub mod conversation;
// pub mod message;
pub mod friend;
