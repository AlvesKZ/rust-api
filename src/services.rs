use actix_web::{
    web::{
        scope, 
        Json,
        Data,
        ServiceConfig,
        Query
    },
    get,
    post,
    HttpResponse, 
    Responder,
};

use serde_json::json;

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

pub fn config(conf: &mut ServiceConfig) {
    let scope = scope("/api")
                    .service(health_checker)
                    .service(create_task)
                    .service(get_all_tasks);  

    conf.service(scope);
}