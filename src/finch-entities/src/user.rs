use sea_orm::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    /// The user ID.
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,

    /// The user's name.
    pub name: String,

    pub team_number: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::team::Entity", from = "Column::TeamNumber", to = "super::team::Column::Number")]
    Team,
}

impl Related<super::team::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Team.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
