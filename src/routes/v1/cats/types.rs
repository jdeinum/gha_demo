use crate::error::Result;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(sqlx::Type, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[sqlx(type_name = "eye_color")]
pub enum EyeColor {
    Blue,
    Brown,
}

#[derive(Serialize, Deserialize, Debug, FromRow, PartialEq, Eq)]
pub struct Cat {
    pub name: String,
    pub cool_cat_club_id: Uuid,
    pub age: i16,
    pub eye_color: EyeColor,
}

impl Cat {
    pub async fn write_to_db(&self, pool: &sqlx::PgPool) -> Result<()> {
        let query = r#"
            INSERT INTO cats (name, cool_cat_club_id, age, eye_color)
            VALUES ($1, $2, $3, $4)
        "#;

        sqlx::query(query)
            .bind(&self.name)
            .bind(self.cool_cat_club_id)
            .bind(self.age)
            .bind(&self.eye_color)
            .execute(pool)
            .await?;

        Ok(())
    }
}
