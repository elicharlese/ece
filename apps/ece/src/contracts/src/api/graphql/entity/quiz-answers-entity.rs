use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "quiz_answers")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub answer_id: i32,
    pub user_id: i32,
    pub quiz_id: i32,
    pub given_answer: String,
    pub is_correct: bool,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl Related<OtherEntity> for Entity {
    fn to() -> RelationDef {
        unimplemented!()
    }
}

impl ActiveModelBehavior for ActiveModel {}