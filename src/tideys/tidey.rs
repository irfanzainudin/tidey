use serde::{Deserialize, Serialize};
type Query = sqlx::Query<'static, sqlx::Sqlite>;
type QueryAs<T> = sqlx::QueryAs<'static, sqlx::Sqlite, T>;

#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct Tidey {
    pub id: i64,
    pub body: String,
    created: i32,
    updated: i32,
}

impl crate::utils::AsRoute for Tidey {
    fn as_route(&self) -> std::borrow::Cow<str> {
        format!("/tideys/{}", self.id).into()
    }
}

impl std::fmt::Display for Tidey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.body)
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct PartialTidey {
    pub body: Option<String>,
}

impl PartialTidey {
    pub fn update_by_id(&self, id: i64) -> Query {
        sqlx::query(
            "UPDATE tideys (body, updated) VALUES (
            COALESCE($1, tideys.body),
            datetime('now')
          ) WHERE id = $2",
        )
        .bind(&self.body)
        .bind(id)
    }

    pub fn create(&self) -> Query {
        sqlx::query(
            "INSERT INTO tideys (body, created, updated) VALUES (
            $1, DATETIME('now'), DATETIME('now')
          )",
        )
        .bind(&self.body)
    }
}

impl Tidey {
    pub fn all() -> QueryAs<Self> {
        sqlx::query_as("SELECT * FROM tideys")
    }

    pub fn last_id() -> QueryAs<(i64,)> {
        sqlx::query_as("SELECT last_insert_rowid()")
    }

    pub fn find_by_id(id: i64) -> QueryAs<Self> {
        sqlx::query_as("SELECT * FROM tideys WHERE id = ?").bind(id)
    }

    pub fn delete_by_id(id: i64) -> Query {
        sqlx::query("DELETE FROM tideys WHERE id = ?").bind(id)
    }

    // pub fn update(&self, partial: PartialTidey) -> Query {
    //     partial.update_by_id(self.id)
    // }
}
