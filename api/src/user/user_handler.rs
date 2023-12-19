use shared::response_models::{Response, ResponseBody};
use application::user::{create, read};
use domain::models::{User, NewUser};
use rocket::{get, post};
use rocket::response::status::{NotFound, Created};
use rocket::serde::json::Json;

#[get("/users")]
pub fn list_users_handler() -> String {
    let users: Vec<User> = read::list_users();
    let response = Response { body: ResponseBody::Users(users) };

    serde_json::to_string(&response).unwrap()
}

#[get("/user/<user_id>")]
pub fn list_user_handler(user_id: i32) -> Result<String, NotFound<String>> {
    let user = read::list_user(user_id)?;
    let response = Response { body: ResponseBody::User(user) };

    Ok(serde_json::to_string(&response).unwrap())
}

#[post("/new_user", format = "application/json", data = "<user>")]
pub fn create_user_handler(user: Json<NewUser>) -> Created<String> {
    create::create_user(user)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = list_users_handler();
        // assert!(Ok(result));
    }
}