use serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;

#[derive(Default, Serialize, Deserialize, Debug, Copy, Clone, EnumIter, DeriveActiveEnum, PartialEq)]
#[sea_orm(rs_type = "i8", db_type = "Integer")]
pub enum Delete {
    #[default]
    #[sea_orm(num_value = 0)]
    No = 0,
    #[sea_orm(num_value = 1)]
    Yes = 1,
}

pub mod user;
// pub mod conversation;
// pub mod message;
pub mod friend;