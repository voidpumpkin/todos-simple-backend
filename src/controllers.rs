use crate::models::EditableTodo;
use crate::models::InsertableTodo;
use crate::models::Todo;
use crate::DbPool;
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};

#[get("/todos")]
pub async fn get_todos(pool: web::Data<DbPool>) -> impl Responder {
    let db = match pool.get() {
        Ok(ok) => ok,
        Err(err) => return HttpResponse::InternalServerError().json(format!("{:#?}", err)),
    };

    let todos_block = web::block(move || Todo::get_all(&db)).await.map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    });

    let todos = match todos_block {
        Ok(ok) => ok,
        Err(err) => return HttpResponse::InternalServerError().json(format!("{:#?}", err)),
    };

    HttpResponse::Ok().json(todos)
}

#[post("/todos")]
pub async fn post_todo(
    pool: web::Data<DbPool>,
    new_todo: web::Json<InsertableTodo>,
) -> impl Responder {
    let db = match pool.get() {
        Ok(ok) => ok,
        Err(err) => return HttpResponse::InternalServerError().json(format!("{:#?}", err)),
    };

    let operation_block = web::block(move || Todo::insert(&db, new_todo.into_inner()))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        });

    match operation_block {
        Ok(()) => {}
        Err(err) => return HttpResponse::InternalServerError().json(format!("{:#?}", err)),
    };

    HttpResponse::Ok().finish()
}

#[patch("/todos/{todo_id}")]
pub async fn patch_todo(
    pool: web::Data<DbPool>,
    web::Path(todo_id): web::Path<i32>,
    edited_todo: web::Json<EditableTodo>,
) -> impl Responder {
    let db = match pool.get() {
        Ok(ok) => ok,
        Err(err) => return HttpResponse::InternalServerError().json(format!("{:#?}", err)),
    };

    let operation_block = web::block(move || Todo::update(&db, todo_id, edited_todo.into_inner()))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        });

    match operation_block {
        Ok(()) => {}
        Err(err) => return HttpResponse::InternalServerError().json(format!("{:#?}", err)),
    };

    HttpResponse::Ok().finish()
}

#[delete("/todos/{todo_id}")]
pub async fn delete_todo(
    pool: web::Data<DbPool>,
    web::Path(todo_id): web::Path<i32>,
) -> impl Responder {
    let db = match pool.get() {
        Ok(ok) => ok,
        Err(err) => return HttpResponse::InternalServerError().json(format!("{:#?}", err)),
    };

    let operation_block = web::block(move || Todo::delete(&db, todo_id))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        });

    match operation_block {
        Ok(()) => {}
        Err(err) => return HttpResponse::InternalServerError().json(format!("{:#?}", err)),
    };

    HttpResponse::Ok().finish()
}

#[delete("/todos")]
pub async fn delete_todos(pool: web::Data<DbPool>) -> impl Responder {
    let db = match pool.get() {
        Ok(ok) => ok,
        Err(err) => return HttpResponse::InternalServerError().json(format!("{:#?}", err)),
    };

    let operation_block = web::block(move || Todo::delete_all(&db))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        });

    match operation_block {
        Ok(()) => {}
        Err(err) => return HttpResponse::InternalServerError().json(format!("{:#?}", err)),
    };

    HttpResponse::Ok().finish()
}
