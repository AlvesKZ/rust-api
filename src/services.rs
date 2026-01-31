use actix_web::{
    web::{
        scope, 
        Json,
        Data,
        ServiceConfig
    },
    get,
    post,
    HttpResponse, 
    Responder,
};

use serde_json::json;

use crate::{AppState, schema::CreateTaskSchema, model::TaskModel};

#[get("/healthchecker")]
async fn health_checker() -> impl Responder {
    const MESSAGE: &str = "Health check: API is up and running smoothly.";

    HttpResponse::Ok().json(json!({
        "status": "success",
        "message": MESSAGE
    }))
}

#[post("/task")]
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
        Err(err) => {
            eprintln!("Database error: {:?}", err);  
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Failed to create task"
            }))
        }
    }
}

pub fn config(conf: &mut ServiceConfig) {
    let scope = scope("/api")
                    .service(health_checker)
                    .service(create_task);

    conf.service(scope);
}
