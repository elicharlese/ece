use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "quizzes")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub quiz_id: i32,
    pub question: String,
    pub correct_answer: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl Related<OtherEntity> for Entity {
    fn to() -> RelationDef {
        unimplemented!()
    }
}

impl ActiveModelBehavior for ActiveModel {}