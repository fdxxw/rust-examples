// JSONを返すのに必要
use rocket_contrib::json::Json;

use crate::models::ToDo;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

/// TODOリストを返す。
/// Jsonの型がResponderをimplしているので、JSON文字列を返すことができる
#[get("/todos")]
pub fn todos() -> Json<Vec<ToDo>> {
    Json(vec![ToDo {
        id: 1,
        title: "Read Rocket tutorial".into(),
        description: "Read https://rocket.rs/guide/quickstart/".into(),
        done: false,
    }])
}


#[get("/todos/<todoid>")]
pub fn todo_by_id(todoid: u32) -> String {
    let todo = ToDo {
        id: todoid,
        title: "Read Rocket tutorial".into(),
        description: "Read https://rocket.rs/guide/quickstart/".into(),
        done: false,
    };
    format!("{:?}", todo)
}


#[post("/todos", data = "<todo>")]
pub fn new_todo(todo: Json<ToDo>) -> String {
    format!("Accepted post request! {:?}", todo.0)
}

