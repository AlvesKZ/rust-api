use actix_web::{
    HttpResponse, Responder, get, post, delete, web::{
        Data, Json, Path, Query, ServiceConfig, scope
    }
};

use serde_json::json;
use uuid::Uuid;

use crate::{AppState, model::TaskModel, schema::{CreateTaskSchema, FilterOptions}};

#[get("/healthchecker")]
async fn health_checker() -> impl Responder {
    const MESSAGE: &str = "Health check: API is up and running smoothly.";

    HttpResponse::Ok().json(json!({
        "status": "success",
        "message": MESSAGE
    }))
}

#[post("/tasks")]
async fn create_task(
    body: Json<CreateTaskSchema>,
    data: Data<AppState>
) -> impl Responder {
    match sqlx::query_as!(
        TaskModel,
        "INSERT INTO tasks (title, content) VALUES ($1, $2) RETURNING *",
        body.title,  
        body.content
    )
    .fetch_one(&data.db)
    .await 
    {
        Ok(task) => {
            HttpResponse::Created().json(json!({ 
                "status": "success",
                "data": task
            }))
        }
        Err(error) => {
            eprintln!("Database error: {:?}", error);  
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Failed to create task"
            }))
        }
    }
}

#[get("/tasks")]
async fn get_all_tasks(
    opts: Query<FilterOptions>,  
    data: Data<AppState>
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit; 

    match sqlx::query_as!(
        TaskModel,
        "SELECT * FROM tasks ORDER BY id LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await 
    {
        Ok(tasks) => {
            let json_response = json!({
                "status": "success", 
                "result": tasks.len(),
                "tasks": tasks
            });

            HttpResponse::Ok().json(json_response)  
        }
        Err(error) => {  
            eprintln!("Database error: {:?}", error);  
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Failed to fetch tasks" 
            }))
        }
    }
}

#[get("/tasks/{id}")]
async fn get_task_by_id(
    path: Path<Uuid>,
    data: Data<AppState>
) -> impl Responder {
    let task_id = path.into_inner();

    match sqlx::query_as!(
        TaskModel,
        "SELECT * FROM tasks WHERE id = $1", 
        task_id
    ) 
    .fetch_one(&data.db)
    .await 
    {
        Ok(task) => {
            let task_response = json!({
                "status": "success",
                "task": task
            });

            HttpResponse::Ok().json(task_response)  
        }
        Err(error) => {  
            eprintln!("Database error: {:?}", error);  
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Failed to fetch task" 
            }))
        }
    }
}

#[delete("/tasks/{id}")]
async fn delete_task_by_id(
    path: Path<Uuid>,
    data: Data<AppState>
) -> impl Responder {
    let task_id = path.into_inner();

    match sqlx::query_as!(
        TaskModel,
        "DELETE FROM tasks WHERE id = $1", 
        task_id
    ) 
    .execute(&data.db)
    .await 
    {
        Ok(_) => {
            HttpResponse::NoContent().finish()  
        }
        Err(error) => {  
            eprintln!("Database error: {:?}", error);  
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Failed to delete task" 
            }))
        }
    }
}

pub fn config(conf: &mut ServiceConfig) {
    let scope = scope("/api")
                    .service(health_checker)
                    .service(create_task)
                    .service(get_all_tasks)
                    .service(get_task_by_id)
                    .service(delete_task_by_id); 
                    
    conf.service(scope);
}