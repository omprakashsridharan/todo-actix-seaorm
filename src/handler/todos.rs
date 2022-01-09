use actix_web::{get, web, Error as ActixError, Responder, Result as ActixResult, Scope};

use crate::AppState;

#[get("/")]
async fn get_todos(state: web::Data<AppState>) -> ActixResult<impl Responder, ActixError> {
    let todos = state.todo_repository.get_todos().await;
    Ok(web::Json(todos))
}

pub fn todos_handler() -> Scope {
    web::scope("/todos").service(get_todos)
}
