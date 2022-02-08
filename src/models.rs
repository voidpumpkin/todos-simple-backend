use crate::diesel::ExpressionMethods;
use crate::schema::todos;
use anyhow::Context;
use diesel::Insertable;
use diesel::QueryDsl;
use diesel::Queryable;
use diesel::RunQueryDsl;
use diesel::SqliteConnection;
use serde::Deserialize;
use serde::Serialize;

#[derive(Queryable, Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub is_checked: bool,
}

#[derive(Insertable, Debug, Deserialize)]
#[table_name = "todos"]
pub struct InsertableTodo {
    pub title: String,
    pub body: String,
    pub is_checked: bool,
}

#[derive(Debug, Deserialize)]
pub struct EditableTodo {
    pub title: Option<String>,
    pub body: Option<String>,
    pub is_checked: Option<bool>,
}

impl Todo {
    pub fn get_all(db: &SqliteConnection) -> anyhow::Result<Vec<Todo>> {
        use crate::schema::todos::dsl::*;

        let results = todos
            .filter(is_checked.eq(false))
            .limit(5)
            .load::<Todo>(db)?;

        Ok(results)
    }
    pub fn insert(db: &SqliteConnection, new_todo: InsertableTodo) -> anyhow::Result<()> {
        use crate::schema::todos::dsl::*;

        diesel::insert_into(todos).values(&new_todo).execute(db)?;

        Ok(())
    }
    pub fn update(
        db: &SqliteConnection,
        todo_id: i32,
        edited_todo: EditableTodo,
    ) -> anyhow::Result<()> {
        let Todo {
            title,
            body,
            is_checked,
            ..
        } = {
            use crate::schema::todos::dsl::*;

            todos
                .filter(id.eq(todo_id))
                .limit(1)
                .load::<Todo>(db)?
                .get(0)
                .cloned()
                .context("Failed to detach the important thing")?
        };

        let todo_title = edited_todo.title.unwrap_or(title);
        let todo_body = edited_todo.body.unwrap_or(body);
        let todo_is_checked = edited_todo.is_checked.unwrap_or(is_checked);

        {
            use crate::schema::todos::dsl::*;
            diesel::update(todos.filter(id.eq(todo_id)))
                .set((
                    title.eq(todo_title),
                    body.eq(todo_body),
                    is_checked.eq(todo_is_checked),
                ))
                .execute(db)?;
        }

        Ok(())
    }
    pub fn delete(db: &SqliteConnection, todo_id: i32) -> anyhow::Result<()> {
        use crate::schema::todos::dsl::*;

        diesel::delete(todos.filter(id.eq(todo_id))).execute(db)?;

        Ok(())
    }
    pub fn delete_all(db: &SqliteConnection) -> anyhow::Result<()> {
        use crate::schema::todos::dsl::*;

        diesel::delete(todos).execute(db)?;

        Ok(())
    }
}
